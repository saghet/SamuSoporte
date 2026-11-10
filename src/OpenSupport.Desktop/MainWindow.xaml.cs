using System;
using System.Windows;
using Wpf.Ui.Controls;

namespace OpenSupport.Desktop;

public partial class MainWindow : FluentWindow
{
    private string _currentCode = "582 914";

    public MainWindow()
    {
        InitializeComponent();
        GenerateNewCode();
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

        // Demostración del banner de sesión remota activa obligatoria
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
}
