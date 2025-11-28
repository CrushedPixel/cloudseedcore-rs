use crate::bridge::cs_format_parameter;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Converts a normalized parameter value to human-readable text.
pub fn format_parameter_value(param_id: ParamId, value: f32) -> String {
    let id: u8 = param_id.into();
    cs_format_parameter(id as u32, value)
}

/// Parameter identifiers matching CloudSeedCore/Parameters.h
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, IntoPrimitive, TryFromPrimitive)]
pub enum ParamId {
    Interpolation = 0,
    LowCutEnabled = 1,
    HighCutEnabled = 2,
    InputMix = 3,
    LowCut = 4,
    HighCut = 5,
    DryOut = 6,
    EarlyOut = 7,
    LateOut = 8,

    TapEnabled = 9,
    TapCount = 10,
    TapDecay = 11,
    TapPredelay = 12,
    TapLength = 13,

    EarlyDiffuseEnabled = 14,
    EarlyDiffuseCount = 15,
    EarlyDiffuseDelay = 16,
    EarlyDiffuseModAmount = 17,
    EarlyDiffuseFeedback = 18,
    EarlyDiffuseModRate = 19,

    LateMode = 20,
    LateLineCount = 21,
    LateDiffuseEnabled = 22,
    LateDiffuseCount = 23,
    LateLineSize = 24,
    LateLineModAmount = 25,
    LateDiffuseDelay = 26,
    LateDiffuseModAmount = 27,
    LateLineDecay = 28,
    LateLineModRate = 29,
    LateDiffuseFeedback = 30,
    LateDiffuseModRate = 31,

    EqLowShelfEnabled = 32,
    EqHighShelfEnabled = 33,
    EqLowpassEnabled = 34,
    EqLowFreq = 35,
    EqHighFreq = 36,
    EqCutoff = 37,
    EqLowGain = 38,
    EqHighGain = 39,
    EqCrossSeed = 40,

    SeedTap = 41,
    SeedDiffusion = 42,
    SeedDelay = 43,
    SeedPostDiffusion = 44,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LateMode {
    Pre = 0,
    Post = 1,
}

/// Contains the state of all CloudSeedCore parameters.
/// All parameters are normalized to the range 0..1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Program {
    // Mix
    pub interpolation: bool,
    pub low_cut_enabled: bool,
    pub high_cut_enabled: bool,
    pub input_mix: f32,
    pub low_cut: f32,
    pub high_cut: f32,
    pub dry_out: f32,
    pub early_out: f32,
    pub late_out: f32,

    // Tap
    pub tap_enabled: bool,
    pub tap_count: f32,
    pub tap_decay: f32,
    pub tap_predelay: f32,
    pub tap_length: f32,

    // Early
    pub early_diffuse_enabled: bool,
    pub early_diffuse_count: f32,
    pub early_diffuse_delay: f32,
    pub early_diffuse_mod_amount: f32,
    pub early_diffuse_feedback: f32,
    pub early_diffuse_mod_rate: f32,

    // Late
    pub late_mode: LateMode,
    pub late_line_count: f32,
    pub late_diffuse_enabled: bool,
    pub late_diffuse_count: f32,
    pub late_line_size: f32,
    pub late_line_mod_amount: f32,
    pub late_diffuse_delay: f32,
    pub late_diffuse_mod_amount: f32,
    pub late_line_decay: f32,
    pub late_line_mod_rate: f32,
    pub late_diffuse_feedback: f32,
    pub late_diffuse_mod_rate: f32,

    // EQ
    pub eq_low_shelf_enabled: bool,
    pub eq_high_shelf_enabled: bool,
    pub eq_lowpass_enabled: bool,
    pub eq_low_freq: f32,
    pub eq_high_freq: f32,
    pub eq_cutoff: f32,
    pub eq_low_gain: f32,
    pub eq_high_gain: f32,
    pub eq_cross_seed: f32,

    // Seeds
    pub seed_tap: f32,
    pub seed_diffusion: f32,
    pub seed_delay: f32,
    pub seed_post_diffusion: f32,
}

/// Conversions between Program and array of normalized parameter values.
impl Program {
    /// Converts this program to an array of normalized parameter values.
    /// You can use this to serialize plugin state.
    pub fn to_array(&self) -> [f32; 45] {
        [
            bool_to_param(self.interpolation),
            bool_to_param(self.low_cut_enabled),
            bool_to_param(self.high_cut_enabled),
            self.input_mix,
            self.low_cut,
            self.high_cut,
            self.dry_out,
            self.early_out,
            self.late_out,
            bool_to_param(self.tap_enabled),
            self.tap_count,
            self.tap_decay,
            self.tap_predelay,
            self.tap_length,
            bool_to_param(self.early_diffuse_enabled),
            self.early_diffuse_count,
            self.early_diffuse_delay,
            self.early_diffuse_mod_amount,
            self.early_diffuse_feedback,
            self.early_diffuse_mod_rate,
            match self.late_mode {
                LateMode::Pre => 0.0,
                LateMode::Post => 1.0,
            },
            self.late_line_count,
            bool_to_param(self.late_diffuse_enabled),
            self.late_diffuse_count,
            self.late_line_size,
            self.late_line_mod_amount,
            self.late_diffuse_delay,
            self.late_diffuse_mod_amount,
            self.late_line_decay,
            self.late_line_mod_rate,
            self.late_diffuse_feedback,
            self.late_diffuse_mod_rate,
            bool_to_param(self.eq_low_shelf_enabled),
            bool_to_param(self.eq_high_shelf_enabled),
            bool_to_param(self.eq_lowpass_enabled),
            self.eq_low_freq,
            self.eq_high_freq,
            self.eq_cutoff,
            self.eq_low_gain,
            self.eq_high_gain,
            self.eq_cross_seed,
            self.seed_tap,
            self.seed_diffusion,
            self.seed_delay,
            self.seed_post_diffusion,
        ]
    }

    /// Creates a `Program` from a full parameter array.
    /// This can be used for deserialization purposes.
    pub fn from_array(a: [f32; 45]) -> Self {
        Self {
            interpolation: param_to_bool(a[0]),
            low_cut_enabled: param_to_bool(a[1]),
            high_cut_enabled: param_to_bool(a[2]),
            input_mix: a[3],
            low_cut: a[4],
            high_cut: a[5],
            dry_out: a[6],
            early_out: a[7],
            late_out: a[8],
            tap_enabled: param_to_bool(a[9]),
            tap_count: a[10],
            tap_decay: a[11],
            tap_predelay: a[12],
            tap_length: a[13],
            early_diffuse_enabled: param_to_bool(a[14]),
            early_diffuse_count: a[15],
            early_diffuse_delay: a[16],
            early_diffuse_mod_amount: a[17],
            early_diffuse_feedback: a[18],
            early_diffuse_mod_rate: a[19],
            late_mode: if a[20] >= 0.5 {
                LateMode::Post
            } else {
                LateMode::Pre
            },
            late_line_count: a[21],
            late_diffuse_enabled: param_to_bool(a[22]),
            late_diffuse_count: a[23],
            late_line_size: a[24],
            late_line_mod_amount: a[25],
            late_diffuse_delay: a[26],
            late_diffuse_mod_amount: a[27],
            late_line_decay: a[28],
            late_line_mod_rate: a[29],
            late_diffuse_feedback: a[30],
            late_diffuse_mod_rate: a[31],
            eq_low_shelf_enabled: param_to_bool(a[32]),
            eq_high_shelf_enabled: param_to_bool(a[33]),
            eq_lowpass_enabled: param_to_bool(a[34]),
            eq_low_freq: a[35],
            eq_high_freq: a[36],
            eq_cutoff: a[37],
            eq_low_gain: a[38],
            eq_high_gain: a[39],
            eq_cross_seed: a[40],
            seed_tap: a[41],
            seed_diffusion: a[42],
            seed_delay: a[43],
            seed_post_diffusion: a[44],
        }
    }

    /// Tries to create a `Program` from a slice of length 45.
    /// Returns None if the slice doesn't have the expected length.
    pub fn from_slice(slice: &[f32]) -> Option<Self> {
        if slice.len() == 45 {
            let mut a = [0.0f32; 45];
            a.copy_from_slice(slice);
            Some(Self::from_array(a))
        } else {
            None
        }
    }
}

/// Get and set parameters by id.
impl Program {
    /// Returns the normalized value of the parameter with the given id.
    pub fn get(&self, id: ParamId) -> f32 {
        match id {
            ParamId::Interpolation => bool_to_param(self.interpolation),
            ParamId::LowCutEnabled => bool_to_param(self.low_cut_enabled),
            ParamId::HighCutEnabled => bool_to_param(self.high_cut_enabled),
            ParamId::InputMix => self.input_mix,
            ParamId::LowCut => self.low_cut,
            ParamId::HighCut => self.high_cut,
            ParamId::DryOut => self.dry_out,
            ParamId::EarlyOut => self.early_out,
            ParamId::LateOut => self.late_out,

            ParamId::TapEnabled => bool_to_param(self.tap_enabled),
            ParamId::TapCount => self.tap_count,
            ParamId::TapDecay => self.tap_decay,
            ParamId::TapPredelay => self.tap_predelay,
            ParamId::TapLength => self.tap_length,

            ParamId::EarlyDiffuseEnabled => bool_to_param(self.early_diffuse_enabled),
            ParamId::EarlyDiffuseCount => self.early_diffuse_count,
            ParamId::EarlyDiffuseDelay => self.early_diffuse_delay,
            ParamId::EarlyDiffuseModAmount => self.early_diffuse_mod_amount,
            ParamId::EarlyDiffuseFeedback => self.early_diffuse_feedback,
            ParamId::EarlyDiffuseModRate => self.early_diffuse_mod_rate,

            ParamId::LateMode => match self.late_mode {
                LateMode::Pre => 0.0,
                LateMode::Post => 1.0,
            },
            ParamId::LateLineCount => self.late_line_count,
            ParamId::LateDiffuseEnabled => bool_to_param(self.late_diffuse_enabled),
            ParamId::LateDiffuseCount => self.late_diffuse_count,
            ParamId::LateLineSize => self.late_line_size,
            ParamId::LateLineModAmount => self.late_line_mod_amount,
            ParamId::LateDiffuseDelay => self.late_diffuse_delay,
            ParamId::LateDiffuseModAmount => self.late_diffuse_mod_amount,
            ParamId::LateLineDecay => self.late_line_decay,
            ParamId::LateLineModRate => self.late_line_mod_rate,
            ParamId::LateDiffuseFeedback => self.late_diffuse_feedback,
            ParamId::LateDiffuseModRate => self.late_diffuse_mod_rate,

            ParamId::EqLowShelfEnabled => bool_to_param(self.eq_low_shelf_enabled),
            ParamId::EqHighShelfEnabled => bool_to_param(self.eq_high_shelf_enabled),
            ParamId::EqLowpassEnabled => bool_to_param(self.eq_lowpass_enabled),

            ParamId::EqLowFreq => self.eq_low_freq,
            ParamId::EqHighFreq => self.eq_high_freq,
            ParamId::EqCutoff => self.eq_cutoff,
            ParamId::EqLowGain => self.eq_low_gain,
            ParamId::EqHighGain => self.eq_high_gain,
            ParamId::EqCrossSeed => self.eq_cross_seed,

            ParamId::SeedTap => self.seed_tap,
            ParamId::SeedDiffusion => self.seed_diffusion,
            ParamId::SeedDelay => self.seed_delay,
            ParamId::SeedPostDiffusion => self.seed_post_diffusion,
        }
    }

    /// Set a parameter by id. Returns previous value on success.
    pub fn set(&mut self, id: ParamId, value: f32) {
        match id {
            ParamId::Interpolation => {
                self.interpolation = param_to_bool(value);
            }
            ParamId::LowCutEnabled => {
                self.low_cut_enabled = param_to_bool(value);
            }
            ParamId::HighCutEnabled => {
                self.high_cut_enabled = param_to_bool(value);
            }
            ParamId::InputMix => {
                self.input_mix = value;
            }
            ParamId::LowCut => {
                self.low_cut = value;
            }
            ParamId::HighCut => {
                self.high_cut = value;
            }
            ParamId::DryOut => {
                self.dry_out = value;
            }
            ParamId::EarlyOut => {
                self.early_out = value;
            }
            ParamId::LateOut => {
                self.late_out = value;
            }
            ParamId::TapEnabled => {
                self.tap_enabled = param_to_bool(value);
            }
            ParamId::TapCount => {
                self.tap_count = value;
            }
            ParamId::TapDecay => {
                self.tap_decay = value;
            }
            ParamId::TapPredelay => {
                self.tap_predelay = value;
            }
            ParamId::TapLength => {
                self.tap_length = value;
            }
            ParamId::EarlyDiffuseEnabled => {
                self.early_diffuse_enabled = param_to_bool(value);
            }
            ParamId::EarlyDiffuseCount => {
                self.early_diffuse_count = value;
            }
            ParamId::EarlyDiffuseDelay => {
                self.early_diffuse_delay = value;
            }
            ParamId::EarlyDiffuseModAmount => {
                self.early_diffuse_mod_amount = value;
            }
            ParamId::EarlyDiffuseFeedback => {
                self.early_diffuse_feedback = value;
            }
            ParamId::EarlyDiffuseModRate => {
                self.early_diffuse_mod_rate = value;
            }
            ParamId::LateMode => {
                self.late_mode = if value >= 0.5 {
                    LateMode::Post
                } else {
                    LateMode::Pre
                };
            }
            ParamId::LateLineCount => {
                self.late_line_count = value;
            }
            ParamId::LateDiffuseEnabled => {
                self.late_diffuse_enabled = param_to_bool(value);
            }
            ParamId::LateDiffuseCount => {
                self.late_diffuse_count = value;
            }
            ParamId::LateLineSize => {
                self.late_line_size = value;
            }
            ParamId::LateLineModAmount => {
                self.late_line_mod_amount = value;
            }
            ParamId::LateDiffuseDelay => {
                self.late_diffuse_delay = value;
            }
            ParamId::LateDiffuseModAmount => {
                self.late_diffuse_mod_amount = value;
            }
            ParamId::LateLineDecay => {
                self.late_line_decay = value;
            }
            ParamId::LateLineModRate => {
                self.late_line_mod_rate = value;
            }
            ParamId::LateDiffuseFeedback => {
                self.late_diffuse_feedback = value;
            }
            ParamId::LateDiffuseModRate => {
                self.late_diffuse_mod_rate = value;
            }
            ParamId::EqLowShelfEnabled => {
                self.eq_low_shelf_enabled = param_to_bool(value);
            }
            ParamId::EqHighShelfEnabled => {
                self.eq_high_shelf_enabled = param_to_bool(value);
            }
            ParamId::EqLowpassEnabled => {
                self.eq_lowpass_enabled = param_to_bool(value);
            }
            ParamId::EqLowFreq => {
                self.eq_low_freq = value;
            }
            ParamId::EqHighFreq => {
                self.eq_high_freq = value;
            }
            ParamId::EqCutoff => {
                self.eq_cutoff = value;
            }
            ParamId::EqLowGain => {
                self.eq_low_gain = value;
            }
            ParamId::EqHighGain => {
                self.eq_high_gain = value;
            }
            ParamId::EqCrossSeed => {
                self.eq_cross_seed = value;
            }
            ParamId::SeedTap => {
                self.seed_tap = value;
            }
            ParamId::SeedDiffusion => {
                self.seed_diffusion = value;
            }
            ParamId::SeedDelay => {
                self.seed_delay = value;
            }
            ParamId::SeedPostDiffusion => {
                self.seed_post_diffusion = value;
            }
        }
    }
}

fn param_to_bool(value: f32) -> bool {
    value >= 0.5
}

fn bool_to_param(value: bool) -> f32 {
    if value {
        1.0
    } else {
        0.0
    }
}

/// "Dark Plate" preset as defined by CloudSeedCore
pub static DARK_PLATE: Program = Program {
    // Mix
    interpolation: true,
    low_cut_enabled: false,
    high_cut_enabled: false,
    input_mix: 0.23469999,
    low_cut: 0.63999999,
    high_cut: 0.29330000,
    dry_out: 0.8706000,
    early_out: 0.0,
    late_out: 0.66139996,

    // Tap
    tap_enabled: false,
    tap_count: 0.19599999,
    tap_decay: 1.0,
    tap_predelay: 0.0,
    tap_length: 0.98670000,

    // Early
    early_diffuse_enabled: false,
    early_diffuse_count: 0.29600000,
    early_diffuse_delay: 0.30669999,
    early_diffuse_mod_amount: 0.14389999,
    early_diffuse_feedback: 0.77069998,
    early_diffuse_mod_rate: 0.24669999,

    // Late
    late_mode: LateMode::Post,
    late_line_count: 1.0,
    late_diffuse_enabled: true,
    late_diffuse_count: 0.48799998,
    late_line_size: 0.46939999,
    late_line_mod_amount: 0.27199998,
    late_diffuse_delay: 0.23999999,
    late_diffuse_mod_amount: 0.14680000,
    late_line_decay: 0.63460000,
    late_line_mod_rate: 0.22929999,
    late_diffuse_feedback: 0.85069996,
    late_diffuse_mod_rate: 0.16669999,

    // EQ
    eq_low_shelf_enabled: false,
    eq_high_shelf_enabled: true,
    eq_lowpass_enabled: false,
    eq_low_freq: 0.38799998,
    eq_high_freq: 0.51339996,
    eq_cutoff: 0.97599995,
    eq_low_gain: 0.55599999,
    eq_high_gain: 0.76800001,
    eq_cross_seed: 0.0,

    // Seeds
    seed_tap: 0.33399999,
    seed_diffusion: 0.18500000,
    seed_delay: 0.21810000,
    seed_post_diffusion: 0.36530000,
};
