// Spike DMX-8: the same tone-to-onset measurement as the Rust probe, with SoundFlow (miniaudio).
// Throwaway code.
//
// Usage:
//   dotnet run -c Release -- list
//   dotnet run -c Release -- latency <output name part|default> <capture name part|loopback> <seconds> [period frames]
//
// "loopback" is WASAPI loopback of the default output (Windows only); on Linux the capture device
// is the output's monitor source ("Monitor of ...").

using System.Collections.Concurrent;
using System.Diagnostics;
using SoundFlow.Abstracts.Devices;
using SoundFlow.Backends.MiniAudio;
using SoundFlow.Backends.MiniAudio.Devices;
using SoundFlow.Enums;
using SoundFlow.Structs;

const double BurstEveryS = 0.5;
const double BurstLenS = 0.02;
const float Amplitude = 0.05f;
const float Threshold = 0.01f;
const double QuietS = 0.2;

if (args.Length < 1)
{
    Console.Error.WriteLine("usage: list | latency <output|default> <capture|loopback> <seconds> [period frames]");
    return 2;
}

using var engine = new MiniAudioEngine();
Console.WriteLine($".NET {Environment.Version}, {System.Runtime.InteropServices.RuntimeInformation.OSDescription}, miniaudio backend {engine.ActiveBackend}");

if (args[0] == "list")
{
    foreach (var d in engine.PlaybackDevices) Console.WriteLine($"output:  {d.Name}{(d.IsDefault ? " (default)" : "")}");
    foreach (var d in engine.CaptureDevices) Console.WriteLine($"capture: {d.Name}{(d.IsDefault ? " (default)" : "")}");
    return 0;
}
if (args[0] != "latency" || args.Length < 4)
{
    Console.Error.WriteLine("usage: list | latency <output|default> <capture|loopback> <seconds> [period frames]");
    return 2;
}

var format = new AudioFormat { Format = SampleFormat.F32, Channels = 2, Layout = ChannelLayout.Stereo, SampleRate = 48000 };
MiniAudioDeviceConfig Config() => new() { PeriodSizeInFrames = args.Length > 4 ? uint.Parse(args[4]) : 0 };

DeviceInfo? Find(DeviceInfo[] list, string part) =>
    part == "default" ? null : list.First(d => d.Name.Contains(part, StringComparison.OrdinalIgnoreCase));

var emitted = new ConcurrentQueue<long>();
var seen = new ConcurrentQueue<long>();
var outBlocks = new ConcurrentQueue<int>();
var inBlocks = new ConcurrentQueue<int>();
double tick = Stopwatch.Frequency;

var outInfo = Find(engine.PlaybackDevices, args[1]);
using var output = engine.InitializePlaybackDevice(outInfo, format, Config());
long every = (long)(BurstEveryS * format.SampleRate), len = (long)(BurstLenS * format.SampleRate);
long frame = 0;
output.OnAudioProcessed += (samples, _) =>
{
    long now = Stopwatch.GetTimestamp();
    int ch = format.Channels, frames = samples.Length / ch;
    outBlocks.Enqueue(frames);
    for (int k = 0; k < frames; k++)
    {
        long pos = frame % every;
        if (pos == 0 && frame > 0) emitted.Enqueue(now + (long)(k * tick / format.SampleRate));
        float v = pos < len && frame >= every ? Amplitude * (float)Math.Sin(Math.Tau * 1000.0 * pos / format.SampleRate) : 0f;
        for (int c = 0; c < ch; c++) samples[k * ch + c] = v;
        frame++;
    }
};

AudioCaptureDevice capture = args[2] == "loopback"
    ? engine.InitializeLoopbackDevice(format, Config())
    : engine.InitializeCaptureDevice(Find(engine.CaptureDevices, args[2]), format, Config());
long quietFrames = (long)(QuietS * format.SampleRate), sinceLoud = long.MaxValue / 2;
capture.OnAudioProcessed += (samples, _) =>
{
    long now = Stopwatch.GetTimestamp();
    int ch = format.Channels, frames = samples.Length / ch;
    inBlocks.Enqueue(frames);
    for (int k = 0; k < frames; k++)
    {
        bool loud = false;
        for (int c = 0; c < ch; c++) loud |= Math.Abs(samples[k * ch + c]) > Threshold;
        if (loud)
        {
            if (sinceLoud > quietFrames) seen.Enqueue(now);
            sinceLoud = 0;
        }
        else sinceLoud++;
    }
};

Console.WriteLine($"output {outInfo?.Name ?? "default"}, capture {(args[2] == "loopback" ? "loopback of default output" : args[2])}, period {(args.Length > 4 ? args[4] : "default")} frames");
int gen0 = GC.CollectionCount(0);
capture.Start();
output.Start();
Thread.Sleep(TimeSpan.FromSeconds(double.Parse(args[3])));
output.Stop();
Thread.Sleep(500);
capture.Stop();

var e = emitted.ToArray();
var delays = new List<double>();
foreach (var s in seen)
{
    var before = e.Where(x => x <= s).DefaultIfEmpty(long.MinValue).Max();
    if (before == long.MinValue) continue;
    double d = (s - before) * 1000.0 / tick;
    if (d < BurstEveryS * 1000.0) delays.Add(d);
}
Console.WriteLine($"bursts emitted {e.Length}, onsets detected {seen.Count}, paired {delays.Count}, GC gen0 {GC.CollectionCount(0) - gen0}");
Stats("  output callback block", "frames", outBlocks.Select(x => (double)x).ToList());
Stats("  capture callback block", "frames", inBlocks.Select(x => (double)x).ToList());
Stats("  tone to detected onset", "ms", delays);
return 0;

static void Stats(string label, string unit, List<double> v)
{
    if (v.Count == 0) { Console.WriteLine($"{label}: no samples"); return; }
    v.Sort();
    double P(double p) => v[(int)Math.Round((v.Count - 1) * p)];
    Console.WriteLine($"{label}: n={v.Count} mean={v.Average():F2} min={v[0]:F2} p50={P(0.5):F2} p95={P(0.95):F2} max={v[^1]:F2} {unit}");
}
