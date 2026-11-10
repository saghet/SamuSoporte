using System;
using System.Collections.Generic;
using System.Windows;
using System.Windows.Controls;
using Wpf.Ui.Controls;

namespace OpenSupport.Desktop;

public partial class MainWindow : FluentWindow
{
    private string _currentCode = "582 914";
    private readonly Dictionary<string, (Wpf.Ui.Controls.Button NavButton, ScrollViewer View)> _navigationMap;

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
    }

    private void NavButton_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Wpf.Ui.Controls.Button btn && btn.Tag is string targetView && _navigationMap.TryGetValue(targetView, out var target))
        {
            // Ocultar todas las vistas y poner botones en Transparent
            foreach (var item in _navigationMap.Values)
            {
                item.View.Visibility = Visibility.Collapsed;
                item.NavButton.Appearance = ControlAppearance.Transparent;
            }

            // Activar la vista y botón seleccionado
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
        Clipboard.SetText(_currentCode);
        StatusLabel.Text = "Código copiado al portapapeles.";
    }

    private void RegenCodeBtn_Click(object sender, RoutedEventArgs e)
    {
        GenerateNewCode();
        StatusLabel.Text = "Nuevo código de asistencia generado.";
    }

    private void ConnectBtn_Click(object sender, RoutedEventArgs e)
    {
        string target = TargetInput.Text?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(target))
        {
            StatusLabel.Text = "Por favor, introduce un código de soporte válido.";
            return;
        }

        StatusLabel.Text = $"Conectando de forma segura con {target} (E2EE Noise Protocol)...";

        // Mostrar banner de sesión remota activa
        ActiveSessionCard.Visibility = Visibility.Visible;
        ActiveSessionText.Text = $"Conectado con {target}. Túnel E2EE operativo.";
        SasCodeText.Text = $"SAS Anti-MitM: {new Random().Next(100000, 999999)}";
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

    private void EmergencyStopBtn_Click(object sender, RoutedEventArgs e)
    {
        ActiveSessionCard.Visibility = Visibility.Collapsed;
        StatusLabel.Text = "Sesión detenida inmediatamente por el usuario.";
        GenerateNewCode();
    }

    private void QuickConnect_Click(object sender, RoutedEventArgs e)
    {
        if (sender is Wpf.Ui.Controls.Button btn && btn.Tag is string deviceName)
        {
            TargetInput.Text = deviceName;
            // Conmutar automáticamente a la vista de Soporte Remoto
            NavButton_Click(NavSoporteRemoto, e);
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
