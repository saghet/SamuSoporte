using System;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using OpenSupport_App.Pages;

namespace OpenSupport_App;

public sealed partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        try
        {
            ExtendsContentIntoTitleBar = true;
            SetTitleBar(AppTitleBar);
        }
        catch (Exception ex)
        {
            System.IO.File.WriteAllText("titlebar_err.txt", ex.ToString());
        }

        Closed += (s, e) =>
        {
            System.IO.File.AppendAllText("launch_log.txt", $"[Closed] Window closed at {DateTime.Now}\n");
        };

        NavFrame.Navigate(typeof(HomePage));
    }

    private void NavView_SelectionChanged(NavigationView sender, NavigationViewSelectionChangedEventArgs args)
    {
        if (args.IsSettingsSelected)
        {
            NavFrame.Navigate(typeof(SettingsPage));
        }
        else if (args.SelectedItemContainer != null)
        {
            switch (args.SelectedItemContainer.Tag.ToString())
            {
                case "home":
                    NavFrame.Navigate(typeof(HomePage));
                    break;
                case "remote":
                    NavFrame.Navigate(typeof(RemoteSupportPage));
                    break;
                case "devices":
                    NavFrame.Navigate(typeof(DevicesPage));
                    break;
                case "console":
                    NavFrame.Navigate(typeof(TechnicianConsolePage));
                    break;
                case "security":
                    NavFrame.Navigate(typeof(SecurityPage));
                    break;
                case "audit":
                    NavFrame.Navigate(typeof(AuditPage));
                    break;
                case "about":
                    NavFrame.Navigate(typeof(AboutPage));
                    break;
            }
        }
    }

    private void TitleBar_BackRequested(TitleBar sender, object args)
    {
        if (NavFrame.CanGoBack)
        {
            NavFrame.GoBack();
        }
    }

    private void TitleBar_PaneToggleRequested(TitleBar sender, object args)
    {
        NavView.IsPaneOpen = !NavView.IsPaneOpen;
    }
}
