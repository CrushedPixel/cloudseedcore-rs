// Rust/C++ bridge for CloudSeedCore using cxx
// the C++ side is implemented in src/cxx/cloudseed_bridge.cpp

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // build script adds src/cxx as include path
        include!("cloudseed_bridge.h");

        type CloudSeedReverb;

        fn cs_new_reverb(sample_rate: f32, max_block: u32) -> UniquePtr<CloudSeedReverb>;
        fn cs_format_parameter(param_id: u32, value: f32) -> String;

        fn reset(self: Pin<&mut CloudSeedReverb>);
        fn set_sample_rate(self: Pin<&mut CloudSeedReverb>, sample_rate: f32);
        fn get_sample_rate(self: &CloudSeedReverb) -> f32;

        fn set_parameter(self: Pin<&mut CloudSeedReverb>, param_id: u32, value: f32);
        fn get_parameter(self: &CloudSeedReverb, param_id: u32) -> f32;

        fn get_all_parameters(self: &CloudSeedReverb, out: &mut [f32]);
        fn load_program(self: Pin<&mut CloudSeedReverb>, params: &[f32]);

        fn process(
            self: Pin<&mut CloudSeedReverb>,
            in_l: &[f32],
            in_r: &[f32],
            out_l: &mut [f32],
            out_r: &mut [f32],
            frames: u32,
        );
    }
}

pub use ffi::*;
