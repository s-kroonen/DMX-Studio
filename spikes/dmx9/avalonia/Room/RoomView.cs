// The room view of the DMX-9 spike in Avalonia: an OpenGlControlBase with the same shaders and
// draw calls as the egui version. Throwaway code.

using System.Diagnostics;
using System.Numerics;
using System.Runtime.InteropServices;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Input;
using Avalonia.OpenGL;
using Avalonia.OpenGL.Controls;
using Avalonia.Themes.Fluent;
using Avalonia.Threading;

namespace Dmx9;

public sealed class App : Application
{
    public override void Initialize() => Styles.Add(new FluentTheme());

    public override void OnFrameworkInitializationCompleted()
    {
        if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            desktop.MainWindow = new Window { Title = "DMX-9 Avalonia room view", Width = 1280, Height = 800, Content = new MainView() };
        else if (ApplicationLifetime is ISingleViewApplicationLifetime single)
            single.MainView = new MainView();
        base.OnFrameworkInitializationCompleted();
    }
}

public sealed class MainView : DockPanel
{
    public MainView()
    {
        var stats = new TextBlock { Margin = new Thickness(8, 4) };
        var help = new TextBlock { Margin = new Thickness(8, 0, 8, 4), Text = "drag to orbit, wheel to zoom, click on the floor or a wall to aim the selected fixtures" };
        var room = new RoomControl(s => stats.Text = s, s => help.Text = s);
        SetDock(stats, Dock.Top);
        SetDock(help, Dock.Top);
        Children.Add(stats);
        Children.Add(help);
        Children.Add(room);
    }
}

public sealed unsafe class RoomControl : OpenGlControlBase
{
    private const int GL_TRIANGLES = 0x0004, GL_FLOAT = 0x1406, GL_ARRAY_BUFFER = 0x8892, GL_STREAM_DRAW = 0x88E0;
    private const int GL_DEPTH_TEST = 0x0B71, GL_BLEND = 0x0BE2, GL_LEQUAL = 0x0203, GL_SRC_ALPHA = 0x0302, GL_ONE = 1;
    private const int GL_COLOR_BUFFER_BIT = 0x4000, GL_DEPTH_BUFFER_BIT = 0x0100, GL_VERTEX_SHADER = 0x8B31, GL_FRAGMENT_SHADER = 0x8B30;

    private readonly Action<string> _stats, _help;
    private int _program, _vao, _vbo, _mvp;
    private delegate* unmanaged<int, int, void> _blendFunc;
    private readonly List<float> _verts = new();
    private float[] _upload = Array.Empty<float>();
    private readonly Camera _cam = new();
    private Vector3? _aim;
    private Point _down, _last;
    private bool _dragging;
    private readonly Stopwatch _clock = Stopwatch.StartNew();
    private double _lastFrame, _windowStart, _logStart;
    private readonly List<double> _frameMs = new(), _buildMs = new(), _logFrames = new(), _logBuild = new();
    private string _info = "";

    public RoomControl(Action<string> stats, Action<string> help)
    {
        _stats = stats;
        _help = help;
        ClipToBounds = true;
    }

    protected override void OnOpenGlInit(GlInterface gl)
    {
        bool es = GlVersion.Type == GlProfileType.OpenGLES;
        string version = es ? "#version 300 es\n" : "#version 330\n";
        const string vs = """
            layout(location = 0) in vec3 a_pos;
            layout(location = 1) in vec4 a_col;
            uniform mat4 u_mvp;
            out vec4 v_col;
            void main() { v_col = a_col; gl_Position = u_mvp * vec4(a_pos, 1.0); }
            """;
        const string fs = """
            precision mediump float;
            in vec4 v_col;
            out vec4 o_col;
            void main() { o_col = v_col; }
            """;
        _program = gl.CreateProgram();
        foreach (var (kind, src) in new[] { (GL_VERTEX_SHADER, vs), (GL_FRAGMENT_SHADER, fs) })
        {
            int s = gl.CreateShader(kind);
            var err = gl.CompileShaderAndGetError(s, version + src);
            if (err != null) throw new InvalidOperationException(err);
            gl.AttachShader(_program, s);
        }
        var linkErr = gl.LinkProgramAndGetError(_program);
        if (linkErr != null) throw new InvalidOperationException(linkErr);
        _mvp = gl.GetUniformLocationString(_program, "u_mvp");
        _vao = gl.GenVertexArray();
        _vbo = gl.GenBuffer();
        gl.BindVertexArray(_vao);
        gl.BindBuffer(GL_ARRAY_BUFFER, _vbo);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 3, GL_FLOAT, 0, 28, IntPtr.Zero);
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1, 4, GL_FLOAT, 0, 28, 12);
        gl.BindVertexArray(0);
        _blendFunc = (delegate* unmanaged<int, int, void>)gl.GetProcAddress("glBlendFunc");
        _info = $"{GlVersion.Type} {GlVersion.Major}.{GlVersion.Minor}";
        Console.WriteLine($"avalonia GL {_info}, .NET {Environment.Version}, {RuntimeInformation.OSDescription}");
    }

    protected override void OnOpenGlRender(GlInterface gl, int fb)
    {
        double now = _clock.Elapsed.TotalMilliseconds;
        Stats(now);
        var size = Bounds.Size;
        double scale = TopLevel.GetTopLevel(this)?.RenderScaling ?? 1;
        int w = Math.Max(1, (int)(size.Width * scale)), h = Math.Max(1, (int)(size.Height * scale));
        float aspect = (float)(size.Width / Math.Max(1, size.Height));

        long t0 = Stopwatch.GetTimestamp();
        var mvp = _cam.ViewProj(aspect);
        int opaque = Scene.Build(_verts, _cam, (float)(now / 1000), _aim);
        if (_upload.Length < _verts.Count) _upload = new float[_verts.Count * 2];
        _verts.CopyTo(_upload);
        double build = Stopwatch.GetElapsedTime(t0).TotalMilliseconds;
        _buildMs.Add(build);
        _logBuild.Add(build);

        gl.BindFramebuffer(0x8D40, fb);
        gl.Viewport(0, 0, w, h);
        gl.ClearColor(0.05f, 0.05f, 0.07f, 1);
        gl.ClearDepth(1);
        gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        gl.UseProgram(_program);
        gl.UniformMatrix4fv(_mvp, 1, false, &mvp);
        gl.BindVertexArray(_vao);
        gl.BindBuffer(GL_ARRAY_BUFFER, _vbo);
        fixed (float* p = _upload)
            gl.BufferData(GL_ARRAY_BUFFER, _verts.Count * sizeof(float), (IntPtr)p, GL_STREAM_DRAW);
        gl.Enable(GL_DEPTH_TEST);
        gl.DepthFunc(GL_LEQUAL);
        gl.Disable(GL_BLEND);
        gl.DrawArrays(GL_TRIANGLES, 0, opaque);
        gl.Enable(GL_BLEND);
        _blendFunc(GL_SRC_ALPHA, GL_ONE);
        gl.DepthMask(0);
        gl.DrawArrays(GL_TRIANGLES, opaque, _verts.Count / 7 - opaque);
        gl.DepthMask(1);
        gl.Disable(GL_DEPTH_TEST);
        gl.Disable(GL_BLEND);
        gl.BindVertexArray(0);
        RequestNextFrameRendering();
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
            string text = $"Avalonia 12.2 with OpenGlControlBase ({_info}): {_frameMs.Count / secs:F1} fps, frame p99 {P99(_frameMs):F1} ms, geometry build {_buildMs.DefaultIfEmpty(0).Average():F2} ms per frame, {_verts.Count / 7} vertices, GC gen0 {GC.CollectionCount(0)}";
            Dispatcher.UIThread.Post(() => _stats(text));
            _frameMs.Clear();
            _buildMs.Clear();
            _windowStart = now;
        }
        if (now - _logStart >= 5000)
        {
            double secs = (now - _logStart) / 1000;
            Console.WriteLine($"avalonia 5 s: {_logFrames.Count / secs:F1} fps, frame p99 {P99(_logFrames):F1} ms, max {_logFrames.DefaultIfEmpty(0).Max():F1} ms, build p99 {P99(_logBuild):F2} ms, max {_logBuild.DefaultIfEmpty(0).Max():F2} ms, GC gen0 {GC.CollectionCount(0)}");
            _logFrames.Clear();
            _logBuild.Clear();
            _logStart = now;
        }
    }

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        _down = _last = e.GetPosition(this);
        _dragging = true;
        e.Pointer.Capture(this);
    }

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        if (!_dragging) return;
        var p = e.GetPosition(this);
        _cam.Yaw -= (float)(p.X - _last.X) * 0.008f;
        _cam.Pitch = Math.Clamp(_cam.Pitch + (float)(p.Y - _last.Y) * 0.008f, 0.05f, 1.5f);
        _last = p;
    }

    protected override void OnPointerReleased(PointerReleasedEventArgs e)
    {
        _dragging = false;
        e.Pointer.Capture(null);
        var p = e.GetPosition(this);
        if (Math.Abs(p.X - _down.X) + Math.Abs(p.Y - _down.Y) > 4) return;
        var size = Bounds.Size;
        float nx = (float)(p.X / size.Width * 2 - 1), ny = (float)(1 - p.Y / size.Height * 2);
        var hit = Scene.Pick(_cam, nx, ny, (float)(size.Width / size.Height));
        if (hit is { } a)
        {
            _aim = a;
            _help($"aim point {a.X:F2} {a.Y:F2} {a.Z:F2}; drag to orbit, wheel to zoom, click to aim");
        }
    }

    protected override void OnPointerWheelChanged(PointerWheelEventArgs e) =>
        _cam.Dist = Math.Clamp(_cam.Dist * (1 - (float)e.Delta.Y * 0.1f), 3f, 30f);
}
