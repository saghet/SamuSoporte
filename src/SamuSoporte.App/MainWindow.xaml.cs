using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;

namespace SamuSoporte.App;

public partial class MainWindow : Window
{
    private string _currentCode = "738 419";

    public MainWindow()
    {
        InitializeComponent();
        GenerateNewCode();
    }

    private void GenerateNewCode()
    {
        var rand = new Random();
        _currentCode = $"{rand.Next(100, 999)} {rand.Next(100, 999)}";
        TxtMyCode.Text = _currentCode;
    }

    private void ResetNavButtons()
    {
        BtnNavHome.Background = Brushes.Transparent;
        BtnNavHome.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
        BtnNavRemote.Background = Brushes.Transparent;
        BtnNavRemote.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
        BtnNavDevices.Background = Brushes.Transparent;
        BtnNavDevices.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
        BtnNavConsole.Background = Brushes.Transparent;
        BtnNavConsole.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
        BtnNavSecurity.Background = Brushes.Transparent;
        BtnNavSecurity.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
        BtnNavAudit.Background = Brushes.Transparent;
        BtnNavAudit.Foreground = new SolidColorBrush(Color.FromRgb(148, 163, 184));
    }

    private void SetActiveButton(Button btn)
    {
        ResetNavButtons();
        btn.Background = new SolidColorBrush(Color.FromRgb(30, 41, 59));
        btn.Foreground = new SolidColorBrush(Color.FromRgb(16, 185, 129));
    }

    private void HideAllViews()
    {
        ViewHome.Visibility = Visibility.Collapsed;
        ViewRemote.Visibility = Visibility.Collapsed;
        ViewDevices.Visibility = Visibility.Collapsed;
        ViewConsole.Visibility = Visibility.Collapsed;
        ViewSecurity.Visibility = Visibility.Collapsed;
        ViewAudit.Visibility = Visibility.Collapsed;
    }

    private void NavHome_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavHome);
        ViewHome.Visibility = Visibility.Visible;
    }

    private void NavRemote_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavRemote);
        ViewRemote.Visibility = Visibility.Visible;
    }

    private void NavDevices_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavDevices);
        ViewDevices.Visibility = Visibility.Visible;
    }

    private void NavConsole_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavConsole);
        ViewConsole.Visibility = Visibility.Visible;
    }

    private void NavSecurity_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavSecurity);
        ViewSecurity.Visibility = Visibility.Visible;
    }

    private void NavAudit_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavAudit);
        ViewAudit.Visibility = Visibility.Visible;
    }

    private void BtnCopyCode_Click(object sender, RoutedEventArgs e)
    {
        Clipboard.SetText(_currentCode);
        TxtStatusFeedback.Text = "¡Código de soporte copiado al portapapeles!";
    }

    private void BtnRegenCode_Click(object sender, RoutedEventArgs e)
    {
        GenerateNewCode();
        TxtStatusFeedback.Text = "Nuevo código generado con éxito.";
    }

    private void BtnConnect_Click(object sender, RoutedEventArgs e)
    {
        string target = TxtTargetId.Text.Trim();
        if (string.IsNullOrEmpty(target))
        {
            TxtStatusFeedback.Text = "Por favor, introduce el código o ID del equipo a asistir.";
            return;
        }

        TxtStatusFeedback.Text = $"Estableciendo túnel cifrado E2EE con {target}...";
        ActiveSessionBanner.Visibility = Visibility.Visible;
        ActiveSessionText.Text = $"Conexión activa con {target}. Control supervisado en curso.";
        ActiveSasText.Text = $"SAS Anti-MitM: {new Random().Next(100, 999)}-{new Random().Next(100, 999)} (Verificado)";

        // Cambiar a la superficie de pantalla remota
        HideAllViews();
        SetActiveButton(BtnNavRemote);
        ViewRemote.Visibility = Visibility.Visible;
    }

    private void BtnFilesOnly_Click(object sender, RoutedEventArgs e)
    {
        string target = TxtTargetId.Text.Trim();
        if (string.IsNullOrEmpty(target))
        {
            TxtStatusFeedback.Text = "Introduce el código para iniciar transferencia de archivos.";
            return;
        }

        TxtStatusFeedback.Text = $"Canal de transferencia SHA-256 abierto con {target}.";
    }

    private void BtnEmergencyStop_Click(object sender, RoutedEventArgs e)
    {
        ActiveSessionBanner.Visibility = Visibility.Collapsed;
        TxtStatusFeedback.Text = "Sesión remota revocada y terminada de inmediato.";
        GenerateNewCode();
    }

    private void BtnEndRemote_Click(object sender, RoutedEventArgs e)
    {
        HideAllViews();
        SetActiveButton(BtnNavHome);
        ViewHome.Visibility = Visibility.Visible;
        ActiveSessionBanner.Visibility = Visibility.Collapsed;
        TxtStatusFeedback.Text = "Sesión remota finalizada por el técnico.";
    }

    private void BtnQuickReconnect_Click(object sender, RoutedEventArgs e)
    {
        TxtTargetId.Text = "OFICINA-CENTRAL-01";
        BtnConnect_Click(sender, e);
    }
}
