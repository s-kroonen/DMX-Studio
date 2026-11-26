// Spike DMX-7: the same update loop as the Rust probe, to compare jitter. Throwaway code.
//
// Usage:
//   dotnet run -c Release -- <port> rate <hz> <channels> <seconds> [sleep|spin]
//   dotnet run -c Release -- <port> sweep <seconds> <hz>
//
// "sleep" waits with Thread.Sleep only; "spin" sleeps most of the period and spins the rest.

using System.Diagnostics;
using System.IO.Ports;
using System.Runtime;

const int TestStart = 99; // 0-based channel 100: no fixture listens there

if (args.Length < 2)
{
    Console.Error.WriteLine("usage: <port> rate <hz> <channels> <seconds> [sleep|spin] | sweep <seconds> <hz>");
    return 2;
}

using var port = new SerialPort(args[0], 38400, Parity.None, 8, StopBits.One)
{
    Handshake = Handshake.None,
    ReadTimeout = 500,
    WriteTimeout = 500,
};
port.Open();
port.DiscardInBuffer();

Console.WriteLine($".NET {Environment.Version}, {System.Runtime.InteropServices.RuntimeInformation.OSDescription}, GC {(GCSettings.IsServerGC ? "server" : "workstation")}, latency mode {GCSettings.LatencyMode}");

switch (args[1])
{
    case "rate":
        Rate(port, double.Parse(args[2]), int.Parse(args[3]), double.Parse(args[4]), args.Length > 5 ? args[5] : "spin");
        break;
    case "sweep":
        Sweep(port, double.Parse(args[2]), double.Parse(args[3]));
        break;
    default:
        Console.Error.WriteLine($"unknown command {args[1]}");
        return 2;
}
return 0;

static double Ms(long ticks) => ticks * 1000.0 / Stopwatch.Frequency;

static bool BlockWrite(SerialPort port, int start, byte[] data, out double ms)
{
    var frame = new byte[4 + data.Length];
    frame[0] = 0xFF;
    frame[1] = (byte)(start & 0xFF);
    frame[2] = (byte)(start >> 8);
    frame[3] = (byte)data.Length;
    data.CopyTo(frame, 4);
    long t = Stopwatch.GetTimestamp();
    port.Write(frame, 0, frame.Length);
    try
    {
        int b;
        do { b = port.ReadByte(); } while (b != 'G' && b >= 0);
        ms = Ms(Stopwatch.GetTimestamp() - t);
        return b == 'G';
    }
    catch (TimeoutException)
    {
        ms = Ms(Stopwatch.GetTimestamp() - t);
        return false;
    }
}

static void WaitUntil(long deadline, string mode)
{
    while (true)
    {
        long left = deadline - Stopwatch.GetTimestamp();
        if (left <= 0) return;
        double leftMs = Ms(left);
        if (mode == "sleep")
        {
            Thread.Sleep(Math.Max(1, (int)leftMs));
        }
        else if (leftMs > 2)
        {
            Thread.Sleep((int)(leftMs - 1));
        }
        else
        {
            Thread.SpinWait(20);
        }
    }
}

static void Stats(string label, List<double> v)
{
    if (v.Count == 0) { Console.WriteLine($"{label}: no samples"); return; }
    v.Sort();
    double P(double p) => v[(int)Math.Round((v.Count - 1) * p)];
    Console.WriteLine($"{label}: n={v.Count} mean={v.Average():F2} p50={P(0.50):F2} p95={P(0.95):F2} p99={P(0.99):F2} max={v[^1]:F2} ms");
}

static void Rate(SerialPort port, double hz, int channels, double seconds, string mode)
{
    long period = (long)(Stopwatch.Frequency / hz);
    int ticks = (int)(seconds * hz);
    var lateness = new List<double>(ticks);
    var writeMs = new List<double>(ticks);
    int overruns = 0, failures = 0;
    int gen0 = GC.CollectionCount(0), gen1 = GC.CollectionCount(1), gen2 = GC.CollectionCount(2);
    long start = Stopwatch.GetTimestamp();
    long next = start;
    for (int i = 0; i < ticks; i++)
    {
        WaitUntil(next, mode);
        lateness.Add(Ms(Stopwatch.GetTimestamp() - next));
        // A fresh array per update, as an engine without pooling would allocate.
        var data = new byte[channels];
        for (int c = 0; c < channels; c++) data[c] = (byte)((i + c) % 256);
        if (BlockWrite(port, TestStart, data, out double ms)) writeMs.Add(ms); else failures++;
        next += period;
        if (Stopwatch.GetTimestamp() > next)
        {
            overruns++;
            next = Stopwatch.GetTimestamp();
        }
    }
    double total = Ms(Stopwatch.GetTimestamp() - start) / 1000.0;
    Console.WriteLine($"target {hz} Hz ({mode}), {channels} channels per update, {ticks} updates in {total:F2} s -> achieved {ticks / total:F1} Hz, overruns {overruns}, failures {failures}, GC gen0/1/2 {GC.CollectionCount(0) - gen0}/{GC.CollectionCount(1) - gen1}/{GC.CollectionCount(2) - gen2}");
    Stats("  wake lateness (jitter)", lateness);
    Stats("  write + acknowledgement", writeMs);
}

// Moving Head 1 (generic-rgbw-moving-head-14ch at address 1): slow 16-bit pan sweep, low dimmer,
// green this time to tell it apart from the Rust run. Ends with the dimmer at 0.
static void Sweep(SerialPort port, double seconds, double hz)
{
    var frame = new byte[14];
    frame[2] = 128; // tilt mid
    frame[5] = 40;  // dimmer, low (usable range 10..134)
    frame[7] = 255; // green
    long period = (long)(Stopwatch.Frequency / hz);
    int ticks = (int)(seconds * hz);
    var writeMs = new List<double>(ticks);
    long next = Stopwatch.GetTimestamp();
    for (int i = 0; i < ticks; i++)
    {
        WaitUntil(next, "spin");
        double t = i / hz;
        double pan = 0.5 + 0.25 * Math.Sin(t * Math.Tau / 10.0);
        ushort value = (ushort)(pan * 65535.0);
        frame[0] = (byte)(value >> 8);
        frame[1] = (byte)(value & 0xFF);
        if (BlockWrite(port, 0, frame, out double ms)) writeMs.Add(ms);
        next += period;
    }
    frame[5] = 0;
    BlockWrite(port, 0, frame, out _);
    Stats($"sweep at {hz} Hz, write + acknowledgement", writeMs);
}
