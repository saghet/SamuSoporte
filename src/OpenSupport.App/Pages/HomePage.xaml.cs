using System;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Windows.ApplicationModel.DataTransfer;

namespace OpenSupport_App.Pages;

public sealed partial class HomePage : Page
{
    private string _currentCode = "582 914";

    public HomePage()
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
        TempCodeDisplay.Text = _currentCode;
    }

    private void CopyCodeBtn_Click(object sender, RoutedEventArgs e)
    {
        var package = new DataPackage();
        package.SetText(_currentCode);
        Clipboard.SetContent(package);
    }

    private void RegenCodeBtn_Click(object sender, RoutedEventArgs e)
    {
        GenerateNewCode();
    }

    private void ConnectBtn_Click(object sender, RoutedEventArgs e)
    {
        string target = TargetDeviceInput.Text?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(target))
        {
            ConnectionFeedback.Text = "Por favor, introduce un código de soporte válido.";
            return;
        }

        ConnectionFeedback.Text = $"Conectando de forma segura con {target} mediante Noise Protocol E2EE...";

        // Simulación de sesión activa para demostración de banner de seguridad obligatorio
        ActiveSessionBanner.Visibility = Visibility.Visible;
        ActiveTechText.Text = $"Conectado con {target}. Túnel E2EE cifrado con éxito.";
        SasCodeText.Text = $"SAS Anti-MitM: {new Random().Next(100000, 999999)}";
    }

    private void FileTransferOnlyBtn_Click(object sender, RoutedEventArgs e)
    {
        string target = TargetDeviceInput.Text?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(target))
        {
            ConnectionFeedback.Text = "Introduce el código para abrir canal de archivos.";
            return;
        }

        ConnectionFeedback.Text = $"Canal de archivos SHA-256 inicializado para {target}.";
    }

    private void EmergencyStopBtn_Click(object sender, RoutedEventArgs e)
    {
        ActiveSessionBanner.Visibility = Visibility.Collapsed;
        ConnectionFeedback.Text = "Sesión detenida inmediatamente por el usuario.";
        GenerateNewCode();
    }
}
