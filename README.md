# cloudseedcore-rs

This crate provides safe Rust bindings to the [CloudSeedCore](https://github.com/GhostNoteAudio/CloudSeedCore/) Reverb
algorithm, commit [`deb21de`](https://github.com/GhostNoteAudio/CloudSeedCore/tree/deb21ded9eb7dad9b3ff94ce1ba96a963716594e).

Building this crate requires a C++14 toolchain.
The C++ bindings are created using [cxx](https://github.com/dtolnay/cxx).

## Example usage

```rs
use cloudseedcore_rs::{ReverbController, ParamId}

// create the reverb instance
let sample_rate = 48_000;
let block_size = 512;
let mut reverb = ReverbController::new(sample_rate, block_size);

// load the "Dark Plate" preset
reverb.set_program(&cloudseedcore_rs::DARK_PLATE);

// only output wet signal
reverb.set_parameter(ParamId::DryOut, 0.0);

// process stereo audio
// assuming you have four sample buffers of size block_size, two for input, two for output:
reverb.process(&in_left, &in_right, &mut out_left, &mut out_right, block_size);

// to persist the reverb's parameter state,
// you can get them as an array of f32 like so:
let state = reverb.get_program().to_array();
```

## License

This crate is MIT licensed.

Any code under `vendor/CloudSeedCore` falls under the original MIT license,
reproduced within that directory.