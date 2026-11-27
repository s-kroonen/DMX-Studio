using System.Diagnostics;
using System.Runtime.Versioning;
using Avalonia;
using Avalonia.Browser;
using Avalonia.Logging;

[assembly: SupportedOSPlatform("browser")]

internal static class Program
{
    private static Task Main(string[] args)
    {
        Trace.Listeners.Add(new ConsoleTraceListener());
        // "?glcontrol" keeps OpenGlControlBase, to show what it does in the browser.
        if (!(args.Length > 0 && args[0].Contains("glcontrol")))
            Dmx9.MainView.ViewFactory = (stats, help) => new Dmx9.WebRoom(stats, help).InInputHost();
        return AppBuilder.Configure<Dmx9.App>().WithInterFont().LogToTrace(LogEventLevel.Warning).StartBrowserAppAsync("out");
    }
}
