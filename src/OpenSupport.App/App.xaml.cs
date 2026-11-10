using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Data;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace OpenSupport_App;

/// <summary>
/// Provides application-specific behavior to supplement the default Application class.
/// </summary>
public partial class App : Application
{
    private Window? _window;
    
    /// <summary>
    /// Initializes the singleton application object.
    /// </summary>
    public App()
    {
        UnhandledException += (s, e) =>
        {
            System.IO.File.WriteAllText("crash_unhandled.txt", $"{e.Message}\n{e.Exception}");
        };

        AppDomain.CurrentDomain.UnhandledException += (s, e) =>
        {
            System.IO.File.WriteAllText("crash_appdomain.txt", $"{e.ExceptionObject}");
        };

        InitializeComponent();
    }

    /// <summary>
    /// Invoked when the application is launched.
    /// </summary>
    /// <param name="args">Details about the launch request and process.</param>
    protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
    {
        try
        {
            System.IO.File.AppendAllText("launch_log.txt", $"[1] OnLaunched invoked at {DateTime.Now}\n");
            _window = new MainWindow();
            System.IO.File.AppendAllText("launch_log.txt", "[2] MainWindow instantiated\n");
            _window.Activate();
            System.IO.File.AppendAllText("launch_log.txt", "[3] MainWindow activated\n");
        }
        catch (Exception ex)
        {
            System.IO.File.WriteAllText("launch_error.txt", ex.ToString());
            throw;
        }
    }
}
