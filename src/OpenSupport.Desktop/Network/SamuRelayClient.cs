using System;
using System.IO;
using System.Net.WebSockets;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;

namespace OpenSupport.Desktop.Network;

public class SamuRelayClient
{
    private ClientWebSocket? _ws;
    private CancellationTokenSource? _cts;
    private readonly Uri _relayUri;

    public string LocalDeviceId { get; }
    public bool IsConnected => _ws != null && _ws.State == WebSocketState.Open;

    public event Action<string, string, string>? SessionRequested; // sessionId, fromDevice, sasCode
    public event Action<string, bool, string>? SessionResponseReceived; // sessionId, accepted, reason
    public event Action<byte[]>? VideoFrameReceived;
    public event Action<string>? StatusChanged;
    public event Action<string>? LogMessage;

    public SamuRelayClient(string relayHost, string deviceId)
    {
        _relayUri = new Uri($"ws://{relayHost}/ws");
        LocalDeviceId = deviceId;
    }

    public async Task ConnectAsync()
    {
        try
        {
            _cts?.Cancel();
            _cts = new CancellationTokenSource();
            _ws = new ClientWebSocket();

            StatusChanged?.Invoke("Conectando al servidor Relay...");
            using var timeoutCts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
            await _ws.ConnectAsync(_relayUri, timeoutCts.Token);

            StatusChanged?.Invoke("Conectado al servidor de Relay.");
            LogMessage?.Invoke($"Conexión establecida con {_relayUri} como Device {LocalDeviceId}");

            // Enviar registro de presencia inicial
            await SendRegisterPresenceAsync();

            // Iniciar bucle de escucha
            _ = Task.Run(() => ReceiveLoopAsync(_cts.Token));
        }
        catch (Exception ex)
        {
            StatusChanged?.Invoke("Desconectado (Relay offline)");
            LogMessage?.Invoke($"Error de conexión a Relay: {ex.Message}");
        }
    }

    private async Task SendRegisterPresenceAsync()
    {
        if (!IsConnected) return;

        var regMsg = new
        {
            type = "Register",
            device_id = LocalDeviceId,
            timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
        };

        await SendJsonAsync(regMsg);
    }

    public async Task RequestSessionAsync(string targetDeviceId, string sessionId, string sasCode)
    {
        if (!IsConnected)
        {
            throw new InvalidOperationException("No hay conexión con el servidor Relay.");
        }

        var req = new
        {
            type = "SessionRequest",
            session_id = sessionId,
            from_device = LocalDeviceId,
            to_device = targetDeviceId,
            sas_code = sasCode,
            timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds()
        };

        await SendJsonAsync(req);
        LogMessage?.Invoke($"Solicitud de sesión enviada a {targetDeviceId} (SAS: {sasCode})");
    }

    public async Task RespondSessionAsync(string sessionId, string toDevice, bool accepted, string reason)
    {
        if (!IsConnected) return;

        var resp = new
        {
            type = "SessionResponse",
            session_id = sessionId,
            from_device = LocalDeviceId,
            to_device = toDevice,
            accepted = accepted,
            reason = reason
        };

        await SendJsonAsync(resp);
        LogMessage?.Invoke($"Respuesta de sesión enviada: accepted={accepted}");
    }

    public async Task SendVideoFrameAsync(string sessionId, string toDevice, byte[] jpegData)
    {
        if (!IsConnected) return;

        var frameMsg = new
        {
            type = "VideoFrame",
            session_id = sessionId,
            from_device = LocalDeviceId,
            to_device = toDevice,
            data_base64 = Convert.ToBase64String(jpegData)
        };

        await SendJsonAsync(frameMsg);
    }

    public async Task SendInputEventAsync(string sessionId, string toDevice, int eventType, int x, int y, int data)
    {
        if (!IsConnected) return;

        var inputMsg = new
        {
            type = "InputEvent",
            session_id = sessionId,
            from_device = LocalDeviceId,
            to_device = toDevice,
            event_type = eventType,
            x = x,
            y = y,
            data = data
        };

        await SendJsonAsync(inputMsg);
    }

    public async Task TerminateSessionAsync(string sessionId, string toDevice, string reason)
    {
        if (!IsConnected) return;

        var termMsg = new
        {
            type = "SessionTermination",
            session_id = sessionId,
            from_device = LocalDeviceId,
            to_device = toDevice,
            reason = reason
        };

        await SendJsonAsync(termMsg);
    }

    private async Task SendJsonAsync(object payload)
    {
        if (_ws == null || _ws.State != WebSocketState.Open) return;

        string json = JsonSerializer.Serialize(payload);
        byte[] bytes = Encoding.UTF8.GetBytes(json);
        await _ws.SendAsync(new ArraySegment<byte>(bytes), WebSocketMessageType.Text, true, CancellationToken.None);
    }

    private async Task ReceiveLoopAsync(CancellationToken ct)
    {
        byte[] buffer = new byte[512 * 1024]; // 512 KB para video frames

        try
        {
            while (!ct.IsCancellationRequested && _ws != null && _ws.State == WebSocketState.Open)
            {
                using var ms = new MemoryStream();
                WebSocketReceiveResult result;

                do
                {
                    result = await _ws.ReceiveAsync(new ArraySegment<byte>(buffer), ct);
                    if (result.MessageType == WebSocketMessageType.Close)
                    {
                        await _ws.CloseAsync(WebSocketCloseStatus.NormalClosure, "Closing", CancellationToken.None);
                        StatusChanged?.Invoke("Conexión de Relay cerrada.");
                        return;
                    }
                    ms.Write(buffer, 0, result.Count);
                }
                while (!result.EndOfMessage);

                ms.Seek(0, SeekOrigin.Begin);
                string text = Encoding.UTF8.GetString(ms.ToArray());
                ProcessIncomingMessage(text);
            }
        }
        catch (OperationCanceledException) { }
        catch (Exception ex)
        {
            LogMessage?.Invoke($"Error en lectura de Relay: {ex.Message}");
            StatusChanged?.Invoke("Desconectado de Relay.");
        }
    }

    private void ProcessIncomingMessage(string json)
    {
        try
        {
            using var doc = JsonDocument.Parse(json);
            var root = doc.RootElement;

            if (!root.TryGetProperty("type", out var typeProp)) return;
            string type = typeProp.GetString() ?? string.Empty;

            switch (type)
            {
                case "SessionRequest":
                {
                    string sessionId = root.GetProperty("session_id").GetString() ?? "";
                    string fromDevice = root.GetProperty("from_device").GetString() ?? "";
                    string sasCode = root.TryGetProperty("sas_code", out var sProp) ? sProp.GetString() ?? "000000" : "000000";
                    SessionRequested?.Invoke(sessionId, fromDevice, sasCode);
                    break;
                }
                case "SessionResponse":
                {
                    string sessionId = root.GetProperty("session_id").GetString() ?? "";
                    bool accepted = root.GetProperty("accepted").GetBoolean();
                    string reason = root.TryGetProperty("reason", out var rProp) ? rProp.GetString() ?? "" : "";
                    SessionResponseReceived?.Invoke(sessionId, accepted, reason);
                    break;
                }
                case "VideoFrame":
                {
                    if (root.TryGetProperty("data_base64", out var dataProp))
                    {
                        string b64 = dataProp.GetString() ?? "";
                        if (!string.IsNullOrEmpty(b64))
                        {
                            byte[] frameBytes = Convert.FromBase64String(b64);
                            VideoFrameReceived?.Invoke(frameBytes);
                        }
                    }
                    break;
                }
            }
        }
        catch (Exception ex)
        {
            LogMessage?.Invoke($"Error procesando mensaje: {ex.Message}");
        }
    }
}
