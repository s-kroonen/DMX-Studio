//! Spike DMX-7: measure the DMX4ALL interface (throughput, acknowledgement delay, update-loop
//! jitter) and drive one moving head for a visual check. Throwaway code.
//!
//! Usage:
//!   dmx7-probe <port> info
//!   dmx7-probe <port> ack                      block sizes 1..255 written at channel 100+
//!   dmx7-probe <port> rate <hz> <channels> <seconds>
//!   dmx7-probe <port> sweep <seconds> <hz>     Moving Head 1 at address 1, low dimmer

use serial2::SerialPort;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Channel 100 and up: no fixture listens there (the v1 venue uses channels 1 to 39).
const TEST_START: u16 = 99; // 0-based

fn open(path: &str) -> SerialPort {
    let mut port = SerialPort::open(path, 38400).expect("open port");
    port.set_read_timeout(Duration::from_millis(500)).expect("timeout");
    port.discard_buffers().ok();
    port
}

/// Read until the interface answers 'G' or the timeout passes. Returns the bytes read.
fn read_reply(port: &mut SerialPort) -> Vec<u8> {
    let mut out = Vec::new();
    let mut buf = [0u8; 64];
    loop {
        match port.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                out.extend_from_slice(&buf[..n]);
                if out.last() == Some(&b'G') {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    out
}

/// Acknowledged block write: FF <start lo> <start hi> <count> <data>, answered with 'G'.
fn block_write(port: &mut SerialPort, start: u16, data: &[u8]) -> Result<Duration, String> {
    assert!(!data.is_empty() && data.len() <= 255);
    let mut frame = Vec::with_capacity(4 + data.len());
    frame.extend_from_slice(&[0xFF, (start & 0xFF) as u8, (start >> 8) as u8, data.len() as u8]);
    frame.extend_from_slice(data);
    let t = Instant::now();
    port.write_all(&frame).map_err(|e| e.to_string())?;
    let reply = read_reply(port);
    let took = t.elapsed();
    if reply.last() == Some(&b'G') {
        Ok(took)
    } else {
        Err(format!("no acknowledgement, got {:?}", reply))
    }
}

fn command(port: &mut SerialPort, cmd: &[u8]) -> (Vec<u8>, Duration) {
    let t = Instant::now();
    port.write_all(cmd).expect("write");
    let reply = read_reply(port);
    (reply, t.elapsed())
}

fn stats(label: &str, mut v: Vec<f64>) {
    if v.is_empty() {
        println!("{label}: no samples");
        return;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| v[((v.len() as f64 - 1.0) * p).round() as usize];
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    println!(
        "{label}: n={} mean={:.2} p50={:.2} p95={:.2} p99={:.2} max={:.2} ms",
        v.len(),
        mean,
        pct(0.50),
        pct(0.95),
        pct(0.99),
        v[v.len() - 1]
    );
}

/// Sleep most of the way, then spin: plain sleep is not precise enough for 22.7 ms periods.
fn wait_until(deadline: Instant) {
    loop {
        let now = Instant::now();
        if now >= deadline {
            return;
        }
        let left = deadline - now;
        if left > Duration::from_millis(2) {
            std::thread::sleep(left - Duration::from_millis(1));
        } else {
            std::hint::spin_loop();
        }
    }
}

fn info(port: &mut SerialPort) {
    for cmd in [&b"C?"[..], b"N?", b"B?"] {
        let (reply, took) = command(port, cmd);
        println!("{:4} -> {:?} ({:.1} ms)", String::from_utf8_lossy(cmd), String::from_utf8_lossy(&reply), took.as_secs_f64() * 1000.0);
    }
}

fn ack(port: &mut SerialPort) {
    for size in [1usize, 8, 16, 39, 64, 128, 255] {
        let mut samples = Vec::new();
        let mut failures = 0;
        for i in 0..50u32 {
            let data = vec![(i % 256) as u8; size];
            match block_write(port, TEST_START, &data) {
                Ok(d) => samples.push(d.as_secs_f64() * 1000.0),
                Err(_) => failures += 1,
            }
        }
        let bytes = 4 + size;
        let wire_ms = bytes as f64 * 10.0 / 38400.0 * 1000.0;
        stats(&format!("block {size:3} ch ({bytes:3} bytes, wire {wire_ms:5.1} ms), failures {failures}"), samples);
    }
}

fn rate(port: &mut SerialPort, hz: f64, channels: usize, seconds: f64) {
    let period = Duration::from_secs_f64(1.0 / hz);
    let ticks = (seconds * hz) as u32;
    let mut lateness = Vec::new();
    let mut write_ms = Vec::new();
    let mut overruns = 0;
    let mut failures = 0;
    let start = Instant::now();
    let mut next = start;
    for i in 0..ticks {
        wait_until(next);
        let woke = Instant::now();
        lateness.push((woke - next).as_secs_f64() * 1000.0);
        let data: Vec<u8> = (0..channels).map(|c| ((i as usize + c) % 256) as u8).collect();
        match block_write(port, TEST_START, &data) {
            Ok(d) => write_ms.push(d.as_secs_f64() * 1000.0),
            Err(_) => failures += 1,
        }
        next += period;
        if Instant::now() > next {
            overruns += 1;
            next = Instant::now(); // drop the missed tick instead of bursting
        }
    }
    let total = start.elapsed().as_secs_f64();
    println!("target {hz} Hz, {channels} channels per update, {ticks} updates in {total:.2} s -> achieved {:.1} Hz, overruns {overruns}, failures {failures}", ticks as f64 / total);
    stats("  wake lateness (jitter)", lateness);
    stats("  write + acknowledgement", write_ms);
}

/// Moving Head 1 (generic-rgbw-moving-head-14ch at address 1): slow 16-bit pan sweep at a low
/// dimmer, blue, tilt fixed. Ends with the dimmer at 0.
fn sweep(port: &mut SerialPort, seconds: f64, hz: f64) {
    // 14 channels: pan, pan fine, tilt, tilt fine, speed, dimmer/strobe, R, G, B, W, macro, macro speed, function, reset
    let mut frame = [0u8; 14];
    frame[2] = 128; // tilt mid
    frame[4] = 0; // pan/tilt speed: fastest, so the head follows the stream
    frame[5] = 40; // dimmer, low (usable range 10..134)
    frame[8] = 255; // blue
    let period = Duration::from_secs_f64(1.0 / hz);
    let ticks = (seconds * hz) as u32;
    let mut next = Instant::now();
    let mut write_ms = Vec::new();
    for i in 0..ticks {
        wait_until(next);
        // Pan from 25 % to 75 % of the range and back, as a sine: one cycle per 10 s.
        let t = i as f64 / hz;
        let pan = 0.5 + 0.25 * (t * std::f64::consts::TAU / 10.0).sin();
        let value = (pan * 65535.0) as u16;
        frame[0] = (value >> 8) as u8;
        frame[1] = (value & 0xFF) as u8;
        if let Ok(d) = block_write(port, 0, &frame) {
            write_ms.push(d.as_secs_f64() * 1000.0);
        }
        next += period;
    }
    frame[5] = 0; // dimmer off
    let _ = block_write(port, 0, &frame);
    stats(&format!("sweep at {hz} Hz, write + acknowledgement"), write_ms);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: dmx7-probe <port> info|ack|rate <hz> <channels> <seconds>|sweep <seconds> <hz>");
        std::process::exit(2);
    }
    let mut port = open(&args[1]);
    match args[2].as_str() {
        "info" => info(&mut port),
        "ack" => ack(&mut port),
        "rate" => rate(&mut port, args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap()),
        "sweep" => sweep(&mut port, args[3].parse().unwrap(), args[4].parse().unwrap()),
        other => eprintln!("unknown command {other}"),
    }
}
