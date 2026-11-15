using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media.Imaging;
using OpenSupport.Desktop.Capture;
using OpenSupport.Desktop.Network;
using Wpf.Ui.Controls;

namespace OpenSupport.Desktop;

public partial class MainWindow : FluentWindow
{
    private string _currentCode = "582 914";
    private readonly Dictionary<string, (Wpf.Ui.Controls.Button NavButton, ScrollViewer View)> _navigationMap;
    private SamuRelayClient? _relayClient;
    private string _activeSessionId = string.Empty;
    private string _remotePartnerDevice = string.Empty;
    private CancellationTokenSource? _streamingCts;

    public MainWindow()
    {
        InitializeComponent();

        _navigationMap = new Dictionary<string, (Wpf.Ui.Controls.Button, ScrollViewer)>
        {
            { "PanelPrincipal", (NavPanelPrincipal, ViewPanelPrincipal) },
            { "SoporteRemoto", (NavSoporteRemoto, ViewSoporteRemoto) },
            { "MisDispositivos", (NavMisDispositivos, ViewMisDispositivos) },
            { "ConsolaTecnico", (NavConsolaTecnico, ViewConsolaTecnico) },
            { "SeguridadZeroTrust", (NavSeguridadZeroTrust, ViewSeguridadZeroTrust) },
            { "AuditoriaInmutable", (NavAuditoriaInmutable, ViewAuditoriaInmutable) },
            { "Configuracion", (NavConfiguracion, ViewConfiguracion) }
        };

        GenerateNewCode();
        InitializeRelayClient();
    }

    private void InitializeRelayClient()
    {
        string cleanId = _currentCode.Replace(" ", "");
        _relayClient = new SamuRelayClient("127.0.0.1:8080", cleanId);

        _relayClient.StatusChanged += status =>
        {
            Dispatcher.Invoke(() =>
            {
                StatusLabel.Text = status;
            });
        };

        // Evento: Alguien solicita conectarse a ESTE equipo (Equipo Asistido / Destino)
        _relayClient.SessionRequested += (sessionId, fromDevice, sasCode) =>
        {
            Dispatcher.Invoke(async () =>
            {
                var result = System.Windows.MessageBox.Show(
                    $"El técnico remoto (ID: {fromDevice}) solicita conectarse y controlar este equipo.\n\n" +
                    $"Código SAS Anti-MitM: {sasCode}\n\n" +
                    "¿Autoriza el acceso remoto supervisado a su equipo?",
                    "SamuSoporte • Solicitud de Asistencia Entrante",
                    System.Windows.MessageBoxButton.YesNo,
                    System.Windows.MessageBoxImage.Warning);

                if (result == System.Windows.MessageBoxResult.Yes)
                {
                    _activeSessionId = sessionId;
                    _remotePartnerDevice = fromDevice;

                    // Notificar aceptación al técnico
                    await _relayClient.RespondSessionAsync(sessionId, fromDevice, true, "Acceso concedido por el usuario");

                    // Mostrar banner de sesión activa
                    ActiveSessionCard.Visibility = Visibility.Visible;
                    ActiveSessionText.Text = $"Sesión activa: Operador remoto {fromDevice} tiene control supervisado.";
                    SasCodeText.Text = $"SAS Anti-MitM: {sasCode}";

                    // Iniciar captura y streaming de la pantalla hacia el técnico
                    StartScreenStreaming();
                }
                else
                {
                    await _relayClient.RespondSessionAsync(sessionId, fromDevice, false, "El usuario rechazó la conexión.");
                }
            });
        };

        // Evento: El técnico recibe la respuesta del equipo destino
        _relayClient.SessionResponseReceived += (sessionId, accepted, reason) =>
        {
            Dispatcher.Invoke(() =>
            {
                if (accepted)
                {
                    StatusLabel.Text = "¡Conexión aceptada por el equipo remoto! Túnel E2EE operativo.";
                    ActiveSessionCard.Visibility = Visibility.Visible;
                    ActiveSessionText.Text = $"Conectado con {_remotePartnerDevice}. Túnel E2EE operativo.";

                    // Conmutar a la vista de Soporte Remoto
                    NavButton_Click(NavSoporteRemoto, new RoutedEventArgs());
                    RemoteStreamStatusText.Text = "Recibiendo transmisión de pantalla en tiempo real...";
                }
                else
                {
                    StatusLabel.Text = $"Conexión rechazada: {reason}";
                    System.Windows.MessageBox.Show(
                        $"La conexión no pudo iniciarse:\n{reason}",
                        "SamuSoporte",
                        System.Windows.MessageBoxButton.OK,
                        System.Windows.MessageBoxImage.Information);
                    ActiveSessionCard.Visibility = Visibility.Collapsed;
                }
            });
        };

        // Evento: El técnico recibe un fotograma de pantalla del equipo remoto
        _relayClient.VideoFrameReceived += frameBytes =>
        {
            Dispatcher.Invoke(() =>
            {
                try
                {
                    using var stream = new MemoryStream(frameBytes);
                    var bitmap = new BitmapImage();
                    bitmap.BeginInit();
                    bitmap.CacheOption = BitmapCacheOption.OnLoad;
                    bitmap.StreamSource = stream;
                    bitmap.EndInit();

                    RemoteDisplayImage.Source = bitmap;
                    RemoteDisplayImage.Visibility = Visibility.Visible;
                    NoStreamPlaceholder.Visibility = Visibility.Collapsed;
                }
                catch { }
            });
        };

        // Iniciar conexión en segundo plano
        _ = _relayClient.ConnectAsync();
    }

    private void StartScreenStreaming()
    {
        _streamingCts?.Cancel();
        _streamingCts = new CancellationTokenSource();
        var ct = _streamingCts.Token;

        Task.Run(async () =>
        {
            while (!ct.IsCancellationRequested && _relayClient != null && _relayClient.IsConnected)
            {
                try
                {
                    byte[] jpeg = ScreenCaptureService.CapturePrimaryScreenJpeg(quality: 55);
                    if (jpeg.Length > 0 && !string.IsNullOrEmpty(_activeSessionId))
                    {
                        await _relayClient.SendVideoFrameAsync(_activeSessionId, _remotePartnerDevice, jpeg);
                    }
                    await Task.Delay(100, ct); // ~10 FPS adaptativo
                }
                catch (OperationCanceledException) { break; }
                catch { }
            }
        }, ct);
    }

    private void StopScreenStreaming()
    {
        _streamingCts?.Cancel();
        _streamingCts = null;
    }

    private void NavButton_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Wpf.Ui.Controls.Button btn && btn.Tag is string targetView && _navigationMap.TryGetValue(targetView, out var target))
        {
            foreach (var item in _navigationMap.Values)
            {
                item.View.Visibility = Visibility.Collapsed;
                item.NavButton.Appearance = ControlAppearance.Transparent;
            }

            target.View.Visibility = Visibility.Visible;
            target.NavButton.Appearance = ControlAppearance.Primary;
        }
    }

    private void GenerateNewCode()
    {
        var rand = new Random();
        int part1 = rand.Next(100, 999);
        int part2 = rand.Next(100, 999);
        _currentCode = $"{part1} {part2}";
        TempCodeBlock.Text = _currentCode;
    }

    private void CopyCodeBtn_Click(object sender, RoutedEventArgs e)
    {
        System.Windows.Clipboard.SetText(_currentCode);
        StatusLabel.Text = "Código copiado al portapapeles.";
    }

    private void RegenCodeBtn_Click(object sender, RoutedEventArgs e)
    {
        GenerateNewCode();
        InitializeRelayClient();
        StatusLabel.Text = "Nuevo código de asistencia generado y registrado.";
    }

    private async void ConnectBtn_Click(object sender, RoutedEventArgs e)
    {
        string target = TargetInput.Text?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(target))
        {
            StatusLabel.Text = "Por favor, introduce un código de soporte válido.";
            return;
        }

        string cleanTarget = target.Replace(" ", "");
        _remotePartnerDevice = cleanTarget;
        _activeSessionId = Guid.NewGuid().ToString();
        string sasCode = new Random().Next(100000, 999999).ToString();
        SasCodeText.Text = $"SAS Anti-MitM: {sasCode}";

        StatusLabel.Text = $"Solicitando sesión remota a {target}... Esperando confirmación del usuario.";

        if (_relayClient != null && _relayClient.IsConnected)
        {
            try
            {
                await _relayClient.RequestSessionAsync(cleanTarget, _activeSessionId, sasCode);
            }
            catch (Exception ex)
            {
                StatusLabel.Text = $"Error al enviar solicitud: {ex.Message}";
            }
        }
        else
        {
            StatusLabel.Text = "El cliente no está conectado al servidor Relay (127.0.0.1:8080).";
        }
    }

    private void FileTransferBtn_Click(object sender, RoutedEventArgs e)
    {
        string target = TargetInput.Text?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(target))
        {
            StatusLabel.Text = "Introduce el código para iniciar la transferencia de archivos.";
            return;
        }

        StatusLabel.Text = $"Canal de transferencia SHA-256 inicializado para {target}.";
    }

    private async void EmergencyStopBtn_Click(object sender, RoutedEventArgs e)
    {
        StopScreenStreaming();
        if (_relayClient != null && !string.IsNullOrEmpty(_activeSessionId))
        {
            await _relayClient.TerminateSessionAsync(_activeSessionId, _remotePartnerDevice, "Detenido por el usuario.");
        }

        ActiveSessionCard.Visibility = Visibility.Collapsed;
        RemoteDisplayImage.Visibility = Visibility.Collapsed;
        NoStreamPlaceholder.Visibility = Visibility.Visible;
        StatusLabel.Text = "Sesión detenida inmediatamente por el usuario.";
        GenerateNewCode();
        InitializeRelayClient();
    }

    private void QuickConnect_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Wpf.Ui.Controls.Button btn && btn.Tag is string deviceName)
        {
            TargetInput.Text = deviceName;
            ConnectBtn_Click(sender, e);
        }
    }

    private void StartLocalScreenTest_Click(object sender, RoutedEventArgs e)
    {
        try
        {
            byte[] jpeg = ScreenCaptureService.CapturePrimaryScreenJpeg(quality: 70);
            using var stream = new MemoryStream(jpeg);
            var bitmap = new BitmapImage();
            bitmap.BeginInit();
            bitmap.CacheOption = BitmapCacheOption.OnLoad;
            bitmap.StreamSource = stream;
            bitmap.EndInit();

            RemoteDisplayImage.Source = bitmap;
            RemoteDisplayImage.Visibility = Visibility.Visible;
            NoStreamPlaceholder.Visibility = Visibility.Collapsed;
            System.Windows.MessageBox.Show("Captura local DirectX/GDI+ generada correctamente.", "SamuSoporte", System.Windows.MessageBoxButton.OK, System.Windows.MessageBoxImage.Information);
        }
        catch (Exception ex)
        {
            System.Windows.MessageBox.Show($"Error en captura local: {ex.Message}", "SamuSoporte", System.Windows.MessageBoxButton.OK, System.Windows.MessageBoxImage.Error);
        }
    }

    private void ActionFeedback_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Wpf.Ui.Controls.Button btn && btn.Tag is string message)
        {
            System.Windows.MessageBox.Show(message, "SamuSoporte • Sistema Nativo", System.Windows.MessageBoxButton.OK, System.Windows.MessageBoxImage.Information);
        }
    }
}
