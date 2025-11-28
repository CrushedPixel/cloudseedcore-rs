use cloudseedcore_rs::{ParamId, ReverbController, DARK_PLATE};
use std::fs;
use std::path::PathBuf;

/// Writes a stereo 32-bit WAV file to `target/reverb_wavs/`.
fn write_stereo_wav(path: &str, sample_rate: u32, left: &[f32], right: &[f32]) {
    let mut p = PathBuf::from("target/reverb_wavs");

    // allow test-specific subpaths
    let sub = PathBuf::from(path);
    if let Some(parent) = sub.parent() {
        p.push(parent);
    }

    fs::create_dir_all(&p).ok();

    let mut p = PathBuf::from("target/reverb_wavs");
    p.push(path);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(p, spec).expect("create wav");
    let n = left.len().min(right.len());
    for i in 0..n {
        writer.write_sample(left[i]).unwrap();
        writer.write_sample(right[i]).unwrap();
    }
    writer.finalize().ok();
}

fn zeros(frames: usize) -> (Vec<f32>, Vec<f32>) {
    (vec![0.0; frames], vec![0.0; frames])
}

fn energy(buf: &[f32]) -> f64 {
    buf.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>()
}

/// Tests whether the [DARK_PLATE] preset generates a reverb tail when excited.
#[test]
fn dark_plate_impulse_generates_reverb_tail() {
    let sample_rate = 48_000.0;
    let block = 512;

    let mut reverb = ReverbController::new(sample_rate, block as u32);

    // initialize to dark plate
    reverb.set_program(&DARK_PLATE);
    // wet output only
    reverb.set_parameter(ParamId::DryOut, 0.0);

    let mut out_l = vec![0.0f32; block];
    let mut out_r = vec![0.0f32; block];

    // accumulate output to be able to save to file
    let mut acc_l = Vec::new();
    let mut acc_r = Vec::new();

    let mut tail_energy = 0.0;

    for i in 0..512 {
        let (in_l, in_r) = if i == 0 {
            // first block: constant signal to ensure strong excitation
            (vec![1.0f32; block], vec![1.0f32; block])
        } else {
            // feed in silence for the rest to capture the full tail
            zeros(block)
        };

        reverb.process(&in_l, &in_r, &mut out_l, &mut out_r, block as u32);
        acc_l.extend_from_slice(&out_l);
        acc_r.extend_from_slice(&out_r);

        tail_energy += energy(&out_l) + energy(&out_r);
    }

    // save to disk for manual inspection
    write_stereo_wav(
        "dark_plate_impulse_tail.wav",
        sample_rate as u32,
        &acc_l,
        &acc_r,
    );

    assert!(
        tail_energy > 1e-6,
        "Expected non-zero reverb tail energy from DARK_PLATE preset, got {tail_energy}"
    );
}
