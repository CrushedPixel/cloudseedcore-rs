use cloudseedcore_rs::{format_parameter_value, parse_parameter_text, ParamId};

/// Tests whether a parameter display value stays the same
/// when going back and forth between the value-to-text
/// and text-to-value conversion functions.
#[test]
fn test_roundtrip_all_params() {
    for param in [
        ParamId::Interpolation,
        ParamId::LowCutEnabled,
        ParamId::HighCutEnabled,
        ParamId::TapEnabled,
        ParamId::LateDiffuseEnabled,
        ParamId::EqLowShelfEnabled,
        ParamId::EqHighShelfEnabled,
        ParamId::EqLowpassEnabled,
        ParamId::EarlyDiffuseEnabled,
        ParamId::InputMix,
        ParamId::EarlyDiffuseFeedback,
        ParamId::TapDecay,
        ParamId::LateDiffuseFeedback,
        ParamId::EqCrossSeed,
        ParamId::SeedTap,
        ParamId::SeedDiffusion,
        ParamId::SeedDelay,
        ParamId::SeedPostDiffusion,
        ParamId::LowCut,
        ParamId::HighCut,
        ParamId::DryOut,
        ParamId::EarlyOut,
        ParamId::LateOut,
        ParamId::TapCount,
        ParamId::TapPredelay,
        ParamId::TapLength,
        ParamId::EarlyDiffuseCount,
        ParamId::EarlyDiffuseDelay,
        ParamId::EarlyDiffuseModAmount,
        ParamId::EarlyDiffuseModRate,
        ParamId::LateMode,
        ParamId::LateLineCount,
        ParamId::LateDiffuseCount,
        ParamId::LateLineSize,
        ParamId::LateLineModAmount,
        ParamId::LateDiffuseDelay,
        ParamId::LateDiffuseModAmount,
        ParamId::LateLineDecay,
        ParamId::LateLineModRate,
        ParamId::LateDiffuseModRate,
        ParamId::EqLowFreq,
        ParamId::EqHighFreq,
        ParamId::EqCutoff,
        ParamId::EqLowGain,
        ParamId::EqHighGain,
    ] {
        for i in 0..=222 {
            let norm_val = i as f32 / 222.0;

            // format to text
            let formatted = format_parameter_value(param, norm_val);

            // parse back
            let parsed = parse_parameter_text(param, &formatted)
                .expect(&format!("could not parse back `{}`", formatted));

            // format again
            let formatted_again = format_parameter_value(param, parsed);

            // compare the formatted versions
            // because the formatting may apply clamping
            // so we can't restore the original exactly for every input

            assert_eq!(
                formatted, formatted_again,
                "Roundtrip mismatch for {:?}: original={} formatted={} parsed={} formatted2={}",
                param, norm_val, formatted, parsed, formatted_again
            );
        }
    }
}
