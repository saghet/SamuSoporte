using System.IO.Pipes;
using System.Security.AccessControl;
using System.Security.Principal;
using System.Text;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace OpenSupport.Service;

public class Worker : BackgroundService
{
    private readonly ILogger<Worker> _logger;
    private const string PipeName = "OpenSupport_SecureIPC";

    public Worker(ILogger<Worker> logger)
    {
        _logger = logger;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _logger.LogInformation("OpenSupport Windows Service iniciado en Session 0 (Mínimo Privilegio).");
        _logger.LogInformation("Iniciando servidor IPC Named Pipe seguro: {PipeName}", PipeName);

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                var pipeSecurity = new PipeSecurity();
                // Permitir acceso estricto al grupo de Administradores y al usuario autenticado local
                var admins = new SecurityIdentifier(WellKnownSidType.BuiltinAdministratorsSid, null);
                var authenticatedUsers = new SecurityIdentifier(WellKnownSidType.AuthenticatedUserSid, null);

                pipeSecurity.AddAccessRule(new PipeAccessRule(admins, PipeAccessRights.FullControl, AccessControlType.Allow));
                pipeSecurity.AddAccessRule(new PipeAccessRule(authenticatedUsers, PipeAccessRights.ReadWrite, AccessControlType.Allow));

                await using var pipeServer = NamedPipeServerStreamAcl.Create(
                    PipeName,
                    PipeDirection.InOut,
                    NamedPipeServerStream.MaxAllowedServerInstances,
                    PipeTransmissionMode.Message,
                    PipeOptions.Asynchronous,
                    0,
                    0,
                    pipeSecurity);

                _logger.LogInformation("Servidor IPC esperando conexión de la interfaz cliente nativa...");
                await pipeServer.WaitForConnectionAsync(stoppingToken);

                _logger.LogInformation("Cliente nativo conectado al canal IPC.");

                var buffer = new byte[4096];
                int bytesRead = await pipeServer.ReadAsync(buffer, stoppingToken);
                if (bytesRead > 0)
                {
                    string message = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                    _logger.LogInformation("Comando recibido en canal IPC: {Command}", message);

                    // Respuesta segura al cliente
                    byte[] response = Encoding.UTF8.GetBytes("{\"status\":\"OK\",\"service\":\"OpenSupportAgent\"}");
                    await pipeServer.WriteAsync(response, stoppingToken);
                    await pipeServer.FlushAsync(stoppingToken);
                }
            }
            catch (OperationCanceledException)
            {
                break;
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error en el ciclo del servidor IPC");
                await Task.Delay(1000, stoppingToken);
            }
        }

        _logger.LogInformation("OpenSupport Windows Service detenido.");
    }
}
