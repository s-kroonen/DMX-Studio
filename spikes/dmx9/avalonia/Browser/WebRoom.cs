// Browser fallback for the DMX-9 room view: OpenGlControlBase has no compositor GL interop in the
// browser, so an HTML canvas sits in the Avalonia layout (NativeControlHost) and room.js draws it
// with WebGL2. Scene, camera and picking stay in C#. Throwaway code.

using System.Diagnostics;
using System.Numerics;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.JavaScript;
using Avalonia.Browser;
using Avalonia.Controls;
using Avalonia.Platform;
using Avalonia.Threading;

namespace Dmx9;

public sealed partial class WebRoom : NativeControlHost
{
    private static WebRoom? _current;
    private readonly Action<string> _stats, _help;
    private readonly Camera _cam = new();
    private readonly List<float> _verts = new();
    private float[] _upload = Array.Empty<float>();
    private Vector3? _aim;
    private readonly Stopwatch _clock = Stopwatch.StartNew();
    private double _lastFrame, _windowStart, _logStart;
    private readonly List<double> _frameMs = new(), _buildMs = new(), _logFrames = new(), _logBuild = new();

    public WebRoom(Action<string> stats, Action<string> help)
    {
        _stats = stats;
        _help = help;
        _current = this;
    }

    protected override IPlatformHandle CreateNativeControlCore(IPlatformHandle parent)
    {
        var canvas = Interop.CreateElement("canvas");
        canvas.SetProperty("style", "width:100%;height:100%;display:block");
        Start(canvas);
        return new JSObjectControlHandle(canvas);
    }

    private static async void Start(JSObject canvas)
    {
        await JSHost.ImportAsync("room.js", "../room.js");
        Interop.Start(canvas);
    }

    /// Called by room.js once per animation frame with the input since the last frame.
    [JSExport]
    public static void Tick(double width, double height, double dx, double dy, double wheel, double clickX, double clickY)
    {
        var self = _current;
        if (self == null) return;
        self.Frame(width, height, dx, dy, wheel, clickX, clickY);
    }

    private void Frame(double width, double height, double dx, double dy, double wheel, double clickX, double clickY)
    {
        double now = _clock.Elapsed.TotalMilliseconds;
        Stats(now);
        float aspect = (float)(width / Math.Max(1, height));
        _cam.Yaw -= (float)dx * 0.008f;
        _cam.Pitch = Math.Clamp(_cam.Pitch + (float)dy * 0.008f, 0.05f, 1.5f);
        _cam.Dist = Math.Clamp(_cam.Dist * (1 + (float)wheel * 0.001f), 3f, 30f);
        if (clickX >= 0)
        {
            var hit = Scene.Pick(_cam, (float)(clickX / width * 2 - 1), (float)(1 - clickY / height * 2), aspect);
            if (hit is { } a)
            {
                _aim = a;
                _help($"aim point {a.X:F2} {a.Y:F2} {a.Z:F2}; drag to orbit, wheel to zoom, click to aim");
            }
        }
        long t0 = Stopwatch.GetTimestamp();
        var mvp = _cam.ViewProj(aspect);
        int opaque = Scene.Build(_verts, _cam, (float)(now / 1000), _aim);
        if (_upload.Length < _verts.Count) _upload = new float[_verts.Count * 2];
        _verts.CopyTo(_upload);
        double build = Stopwatch.GetElapsedTime(t0).TotalMilliseconds;
        _buildMs.Add(build);
        _logBuild.Add(build);
        var m = new double[16];
        var mf = MemoryMarshal.CreateSpan(ref mvp.M11, 16);
        for (int i = 0; i < 16; i++) m[i] = mf[i];
        Interop.Draw(MemoryMarshal.AsBytes(_upload.AsSpan(0, _verts.Count)), opaque, m);
    }

    private static double P99(List<double> v)
    {
        if (v.Count == 0) return 0;
        v.Sort();
        return v[(int)Math.Round((v.Count - 1) * 0.99)];
    }

    private void Stats(double now)
    {
        double dt = now - _lastFrame;
        _lastFrame = now;
        _frameMs.Add(dt);
        _logFrames.Add(dt);
        if (now - _windowStart >= 1000)
        {
            double secs = (now - _windowStart) / 1000;
            string text = $"Avalonia 12.2 browser, WebGL2 canvas through NativeControlHost: {_frameMs.Count / secs:F1} fps, frame p99 {P99(_frameMs):F1} ms, geometry build {_buildMs.DefaultIfEmpty(0).Average():F2} ms per frame, {_verts.Count / 7} vertices, GC gen0 {GC.CollectionCount(0)}";
            Dispatcher.UIThread.Post(() => _stats(text));
            _frameMs.Clear();
            _buildMs.Clear();
            _windowStart = now;
        }
        if (now - _logStart >= 5000)
        {
            double secs = (now - _logStart) / 1000;
            Console.WriteLine($"avalonia web 5 s: {_logFrames.Count / secs:F1} fps, frame p99 {P99(_logFrames):F1} ms, max {_logFrames.DefaultIfEmpty(0).Max():F1} ms, build p99 {P99(_logBuild):F2} ms, max {_logBuild.DefaultIfEmpty(0).Max():F2} ms, GC gen0 {GC.CollectionCount(0)}");
            _logFrames.Clear();
            _logBuild.Clear();
            _logStart = now;
        }
    }

    private static partial class Interop
    {
        [JSImport("globalThis.document.createElement")]
        public static partial JSObject CreateElement(string tagName);

        [JSImport("start", "room.js")]
        public static partial void Start(JSObject canvas);

        [JSImport("draw", "room.js")]
        public static partial void Draw([JSMarshalAs<JSType.MemoryView>] Span<byte> vertices, int opaque, double[] mvp);
    }
}
