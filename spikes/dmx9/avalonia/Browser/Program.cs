using System.Runtime.Versioning;
using Avalonia;
using Avalonia.Browser;

[assembly: SupportedOSPlatform("browser")]

internal static class Program
{
    private static Task Main(string[] args) =>
        AppBuilder.Configure<Dmx9.App>().WithInterFont().StartBrowserAppAsync("out");
}
