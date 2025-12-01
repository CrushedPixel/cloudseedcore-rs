use num_enum::{IntoPrimitive, TryFromPrimitive};

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
