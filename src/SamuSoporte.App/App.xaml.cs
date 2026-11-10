using System;
using System.IO;
using System.Windows;

namespace SamuSoporte.App;

public partial class App : Application
{
    private static void Log(string message)
    {
        try
        {
            var logStart = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "samusoporte_startup.log");
            File.AppendAllText(logStart, $"[{DateTime.Now:HH:mm:ss.fff}] {message}\n");
        }
        catch { }
    }

    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);

        Log("App.OnStartup enter");

        AppDomain.CurrentDomain.UnhandledException += (s, args) =>
        {
            Log($"AppDomain UnhandledException: {args.ExceptionObject}");
            var logPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "samusoporte_appdomain_crash.txt");
            File.WriteAllText(logPath, $"{args.ExceptionObject}");
        };

        DispatcherUnhandledException += (s, args) =>
        {
            Log($"DispatcherUnhandledException: {args.Exception}");
            var logPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "samusoporte_wpf_crash.txt");
            File.WriteAllText(logPath, $"{args.Exception}");
            MessageBox.Show($"Error en SamuSoporte: {args.Exception.Message}\n\nDetalles guardados en {logPath}", "SamuSoporte", MessageBoxButton.OK, MessageBoxImage.Error);
            args.Handled = true;
        };

        try
        {
            Log("Creating MainWindow instance...");
            var mainWindow = new MainWindow();
            
            mainWindow.Loaded += (s, ev) => Log("MainWindow Loaded event fired");
            mainWindow.Closing += (s, ev) => Log($"MainWindow Closing event fired (Cancel={ev.Cancel})");
            mainWindow.Closed += (s, ev) => Log("MainWindow Closed event fired");

            Current.MainWindow = mainWindow;
            Log("Showing MainWindow...");
            mainWindow.Show();
            Log("MainWindow.Show() completed successfully");

            // Ensure dispatcher keeps running and listen to unhandled exceptions on Dispatcher
            Current.Dispatcher.UnhandledException += (s, ev) =>
            {
                Log($"Current.Dispatcher UnhandledException: {ev.Exception}");
            };
        }
        catch (Exception ex)
        {
            Log($"Exception during Startup: {ex}");
            var logPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "samusoporte_startup_error.txt");
            File.WriteAllText(logPath, $"{ex}");
            MessageBox.Show($"Error crítico iniciando SamuSoporte:\n{ex.Message}", "SamuSoporte", MessageBoxButton.OK, MessageBoxImage.Error);
            Shutdown(1);
        }
    }

    protected override void OnExit(ExitEventArgs e)
    {
        Log($"App.OnExit enter (ExitCode: {e.ApplicationExitCode})");
        base.OnExit(e);
    }
}

