use crate::bridge::*;
use crate::{ParamId, Program};

/// A stereo reverb.
///
/// This is a safe wrapper around the CloudSeedCore ReverbController.
pub struct ReverbController {
    inner: cxx::UniquePtr<CloudSeedReverb>,
    max_block_size: u32,
}

// SAFETY: the underlying CloudSeedCore ReverbController written in C++ is single-threaded.
unsafe impl Send for ReverbController {}
unsafe impl Sync for ReverbController {}

impl ReverbController {
    /// Creates a reverb instance with the given sample rate
    /// and maximum block size that will be passed to `process`.
    pub fn new(sample_rate: f32, max_block_size: u32) -> Self {
        let inner = cs_new_reverb(sample_rate, max_block_size);
        Self {
            inner,
            max_block_size,
        }
    }

    /// Returns the maximum block size that can be passed to `process` for this instance.
    pub fn max_block_size(&self) -> u32 {
        self.max_block_size
    }

    /// Processes a stereo signal.
    /// All buffers must be at least `num_samples` long.
    /// Panics if `num_samples` is greater than `max_block_size`.
    pub fn process(
        &mut self,
        in_l: &[f32],
        in_r: &[f32],
        out_l: &mut [f32],
        out_r: &mut [f32],
        num_samples: u32,
    ) {
        assert!(num_samples <= self.max_block_size);

        self.inner
            .as_mut()
            .unwrap()
            .process(in_l, in_r, out_l, out_r, num_samples);
    }

    /// Clears internal buffers, ending any ongoing reverb tail.
    pub fn reset(&mut self) {
        self.inner.as_mut().unwrap().reset();
    }

    /// Updates the reverb's sample rate in Hz.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.inner.as_mut().unwrap().set_sample_rate(sample_rate);
    }

    /// Applies a normalized parameter value in range 0..1
    /// to the parameter with the given id.
    pub fn set_parameter(&mut self, id: ParamId, value: f32) {
        let id: u8 = id.into();
        self.inner.as_mut().unwrap().set_parameter(id as u32, value);
    }

    /// Returns the normalized parameter value in range 0..1 for the given parameter id.
    pub fn get_parameter(&self, id: ParamId) -> f32 {
        let id: u8 = id.into();
        self.inner.as_ref().unwrap().get_parameter(id as u32)
    }

    /// Returns a snapshot of all current parameter values.
    /// The returned [Program] can be used to serialize parameter state.
    pub fn get_program(&self) -> Program {
        // fill a temporary array using the cxx bridge, then convert to typed Program
        let mut vals = [0.0f32; 45];
        self.inner.as_ref().unwrap().get_all_parameters(&mut vals);
        Program::from_array(vals)
    }

    /// Loads a program, ending any ongoing reverb tail.
    pub fn set_program(&mut self, program: &Program) {
        let params = program.to_array();
        self.inner.as_mut().unwrap().load_program(&params);
    }
}
