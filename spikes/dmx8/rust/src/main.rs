//! Spike DMX-8: delay from a test tone to its detected onset in a loopback capture. Throwaway code.
//!
//! The probe plays a short 1 kHz burst every 500 ms on an output device and captures the same
//! audio back: on Linux from the output's monitor source, on Windows through WASAPI loopback (an
//! output device opened as input). Delay = time the onset is seen in a capture callback minus the
//! time the burst was handed to the output callback.
//!
//! Usage:
//!   dmx8-probe hosts
//!   dmx8-probe <host> list
//!   dmx8-probe <host> latency <output name part|default> <capture name part> <seconds> [buffer frames]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, Host, StreamConfig};
use std::sync::mpsc;
use std::time::{Duration, Instant};

const BURST_EVERY_S: f64 = 0.5;
const BURST_LEN_S: f64 = 0.02;
const AMPLITUDE: f32 = 0.05;
const THRESHOLD: f32 = 0.01;
const QUIET_S: f64 = 0.2;

fn host(name: &str) -> Host {
    let id = cpal::available_hosts()
        .into_iter()
        .find(|h| h.name().eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("host {name} not available; see `hosts`"));
    cpal::host_from_id(id).expect("open host")
}

fn name(d: &Device) -> String {
    d.description().map(|d| d.name().to_string()).unwrap_or_else(|_| "?".into())
}

fn list(host: &Host) {
    for d in host.devices().expect("devices") {
        let input = d.default_input_config().map(|c| format!("{} ch {} Hz {:?}", c.channels(), c.sample_rate(), c.sample_format())).ok();
        let output = d.default_output_config().map(|c| format!("{} ch {} Hz {:?}", c.channels(), c.sample_rate(), c.sample_format())).ok();
        println!("{:60} in: {:28} out: {}", name(&d), input.unwrap_or("-".into()), output.unwrap_or("-".into()));
    }
}

fn find(host: &Host, part: &str, input: bool) -> Device {
    if part == "default" {
        return if input { host.default_input_device() } else { host.default_output_device() }.expect("default device");
    }
    host.devices()
        .expect("devices")
        .find(|d| {
            name(d).to_lowercase().contains(&part.to_lowercase())
                && if input { d.default_input_config().is_ok() } else { d.default_output_config().is_ok() }
        })
        .unwrap_or_else(|| panic!("no {} device matches {part:?}", if input { "capture" } else { "output" }))
}

fn stats(label: &str, unit: &str, mut v: Vec<f64>) {
    if v.is_empty() {
        println!("{label}: no samples");
        return;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| v[((v.len() as f64 - 1.0) * p).round() as usize];
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    println!("{label}: n={} mean={mean:.2} min={:.2} p50={:.2} p95={:.2} max={:.2} {unit}", v.len(), v[0], pct(0.5), pct(0.95), v[v.len() - 1]);
}

fn latency(host: &Host, out_part: &str, in_part: &str, seconds: f64, frames: Option<u32>) {
    let out_dev = find(host, out_part, false);
    let in_dev = find(host, in_part, true);
    let out_cfg = out_dev.default_output_config().expect("output config");
    let in_cfg = in_dev.default_input_config().expect("input config");
    let buffer_size = frames.map(BufferSize::Fixed).unwrap_or(BufferSize::Default);
    let out_conf = StreamConfig { channels: out_cfg.channels(), sample_rate: out_cfg.sample_rate(), buffer_size };
    let in_conf = StreamConfig { channels: in_cfg.channels(), sample_rate: in_cfg.sample_rate(), buffer_size };
    println!("host {}, output {:?} {:?}, capture {:?} {:?}", host.id().name(), name(&out_dev), out_conf, name(&in_dev), in_conf);

    let (emit_tx, emit_rx) = mpsc::channel::<Instant>();
    let (seen_tx, seen_rx) = mpsc::channel::<Instant>();
    let (out_blk_tx, out_blk_rx) = mpsc::channel::<usize>();
    let (in_blk_tx, in_blk_rx) = mpsc::channel::<usize>();

    let out_ch = out_conf.channels as usize;
    let out_sr = out_conf.sample_rate as f64;
    let every = (BURST_EVERY_S * out_sr) as u64;
    let len = (BURST_LEN_S * out_sr) as u64;
    let mut frame: u64 = 0;
    let out_stream = out_dev
        .build_output_stream(
            out_conf,
            move |data: &mut [f32], _| {
                let now = Instant::now();
                let _ = out_blk_tx.send(data.len() / out_ch);
                for (k, chunk) in data.chunks_mut(out_ch).enumerate() {
                    let pos = frame % every;
                    if pos == 0 && frame > 0 {
                        let _ = emit_tx.send(now + Duration::from_secs_f64(k as f64 / out_sr));
                    }
                    let v = if pos < len && frame >= every {
                        AMPLITUDE * (std::f64::consts::TAU * 1000.0 * pos as f64 / out_sr).sin() as f32
                    } else {
                        0.0
                    };
                    chunk.fill(v);
                    frame += 1;
                }
            },
            |e| eprintln!("output error: {e}"),
            None,
        )
        .expect("build output stream");

    let in_ch = in_conf.channels as usize;
    let quiet_frames = (QUIET_S * in_conf.sample_rate as f64) as u64;
    let mut since_loud: u64 = u64::MAX / 2;
    let in_stream = in_dev
        .build_input_stream(
            in_conf,
            move |data: &[f32], _| {
                let now = Instant::now();
                let _ = in_blk_tx.send(data.len() / in_ch);
                for chunk in data.chunks(in_ch) {
                    let loud = chunk.iter().any(|s| s.abs() > THRESHOLD);
                    if loud {
                        if since_loud > quiet_frames {
                            let _ = seen_tx.send(now);
                        }
                        since_loud = 0;
                    } else {
                        since_loud += 1;
                    }
                }
            },
            |e| eprintln!("capture error: {e}"),
            None,
        )
        .expect("build input stream");

    in_stream.play().expect("start capture");
    out_stream.play().expect("start output");
    std::thread::sleep(Duration::from_secs_f64(seconds));
    drop(out_stream);
    std::thread::sleep(Duration::from_millis(500));
    drop(in_stream);

    let emitted: Vec<Instant> = emit_rx.try_iter().collect();
    let seen: Vec<Instant> = seen_rx.try_iter().collect();
    let mut delays = Vec::new();
    for s in &seen {
        if let Some(e) = emitted.iter().rev().find(|e| *e <= s) {
            let d = (*s - *e).as_secs_f64() * 1000.0;
            if d < BURST_EVERY_S * 1000.0 {
                delays.push(d);
            }
        }
    }
    println!("bursts emitted {}, onsets detected {}, paired {}", emitted.len(), seen.len(), delays.len());
    stats("  output callback block", "frames", out_blk_rx.try_iter().map(|n| n as f64).collect());
    stats("  capture callback block", "frames", in_blk_rx.try_iter().map(|n| n as f64).collect());
    stats("  tone to detected onset", "ms", delays);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("hosts") => {
            for h in cpal::available_hosts() {
                println!("{}", h.name());
            }
            println!("default: {}", cpal::default_host().id().name());
        }
        Some(h) if args.get(2).map(String::as_str) == Some("list") => list(&host(h)),
        Some(h) if args.get(2).map(String::as_str) == Some("latency") && args.len() >= 6 => latency(
            &host(h),
            &args[3],
            &args[4],
            args[5].parse().expect("seconds"),
            args.get(6).map(|f| f.parse().expect("buffer frames")),
        ),
        _ => {
            eprintln!("usage: dmx8-probe hosts | <host> list | <host> latency <output|default> <capture> <seconds> [buffer frames]");
            std::process::exit(2);
        }
    }
}
