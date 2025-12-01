use crate::bridge::cs_format_parameter;
use crate::params::id::ParamId;
use lexical_parse_float::FromLexical;

/// Converts a normalized parameter value to human-readable text.
pub fn format_parameter_value(param_id: ParamId, value: f32) -> String {
    let id: u8 = param_id.into();
    cs_format_parameter(id as u32, value)
}

/// Converts human-readable text into a normalized parameter value.
pub fn parse_parameter_text(param: ParamId, text: &str) -> Option<f32> {
    let t = text.trim().to_uppercase();

    match param {
        // booleans
        ParamId::Interpolation
        | ParamId::LowCutEnabled
        | ParamId::HighCutEnabled
        | ParamId::TapEnabled
        | ParamId::LateDiffuseEnabled
        | ParamId::EqLowShelfEnabled
        | ParamId::EqHighShelfEnabled
        | ParamId::EqLowpassEnabled
        | ParamId::EarlyDiffuseEnabled => Some(if t.contains("ENABLED") { 1.0 } else { 0.0 }),

        // linear percent
        ParamId::InputMix
        | ParamId::EarlyDiffuseFeedback
        | ParamId::TapDecay
        | ParamId::LateDiffuseFeedback
        | ParamId::EqCrossSeed => Some((extract_number(&t)? / 100.0).clamp(0.0, 1.0)),

        // seeds
        ParamId::SeedTap
        | ParamId::SeedDiffusion
        | ParamId::SeedDelay
        | ParamId::SeedPostDiffusion => Some((extract_number(&t)? / 999.999).clamp(0.0, 1.0)),

        // low freq
        ParamId::LowCut => {
            let hz = extract_number(&t)?;
            let x = (hz - 20.0) / 980.0;
            Some(inv_resp4oct(x))
        }

        // high freq
        ParamId::HighCut | ParamId::EqHighFreq | ParamId::EqCutoff => {
            let hz = extract_number(&t)?;
            let x = (hz - 400.0) / 19600.0;
            Some(inv_resp4oct(x))
        }

        // output level
        ParamId::DryOut | ParamId::EarlyOut | ParamId::LateOut => {
            if t.contains("MUTED") {
                return Some(0.0);
            }
            let db = extract_number(&t)?;
            Some((db + 30.0) / 30.0)
        }

        ParamId::TapCount => {
            let n = extract_number(&t)?;
            Some(((n - 1.0) / 255.0).clamp(0.0, 1.0))
        }

        ParamId::TapPredelay => {
            let ms = extract_number(&t)?;
            Some(inv_resp1dec(ms / 500.0))
        }

        ParamId::TapLength => {
            let ms = extract_number(&t)?;
            Some((ms - 10.0) / 990.0)
        }

        ParamId::EarlyDiffuseCount => {
            let n = extract_number(&t)?;
            Some((n - 1.0) / 11.999)
        }

        ParamId::EarlyDiffuseDelay => {
            let ms = extract_number(&t)?;
            Some((ms - 10.0) / 90.0)
        }

        ParamId::EarlyDiffuseModAmount => Some((extract_number(&t)? / 100.0) / 2.5),

        ParamId::EarlyDiffuseModRate => {
            let hz = extract_number(&t)?;
            Some(inv_resp2dec(hz / 5.0))
        }

        ParamId::LateMode => Some(if t.contains("POST") { 1.0 } else { 0.0 }),

        ParamId::LateLineCount => {
            let n = extract_number(&t)?;
            Some((n - 1.0) / 11.999)
        }

        ParamId::LateDiffuseCount => {
            let n = extract_number(&t)?;
            Some((n - 1.0) / 7.999)
        }

        ParamId::LateLineSize => {
            let ms = extract_number(&t)?;
            let x = (ms - 20.0) / 980.0;
            Some(inv_resp2dec(x))
        }

        ParamId::LateLineModAmount | ParamId::LateDiffuseModAmount => {
            Some((extract_number(&t)? / 100.0) / 2.5)
        }

        ParamId::LateDiffuseDelay => {
            let ms = extract_number(&t)?;
            Some((ms - 10.0) / 90.0)
        }

        ParamId::LateLineDecay => {
            let n = extract_number(&t)?;
            let sec = if t.contains("MS") { n / 1000.0 } else { n };
            let base = (sec - 0.05) / 59.95;
            Some(inv_resp3dec(base))
        }

        ParamId::LateLineModRate | ParamId::LateDiffuseModRate => {
            let hz = extract_number(&t)?;
            Some(inv_resp2dec(hz / 5.0))
        }

        ParamId::EqLowFreq => {
            let hz = extract_number(&t)?;
            let x = (hz - 20.0) / 980.0;
            Some(inv_resp3oct(x))
        }

        ParamId::EqLowGain | ParamId::EqHighGain => {
            let db = extract_number(&t)?;
            Some((db + 20.0) / 20.0)
        }
    }
}

/// Extracts the first number from anywhere in the string.
/// Supports -12.5, +3.5, 5., .5, etc.
fn extract_number(input: &str) -> Option<f32> {
    // ignore any text before the number
    let start_index = input.find(|c: char| c.is_numeric() || c == '.' || c == '-')?;

    // parse into f32
    f32::from_lexical_partial((&input[start_index..]).as_bytes())
        .ok()
        .map(|i| i.0)
}

// the corresponding inverse functions to the
// "Resp" functions declared in Cloudseed/DSP/Utils.h

const DEC1_MULT: f32 = (10.0 / 9.0) * 0.1;
const DEC2_MULT: f32 = (100.0 / 99.0) * 0.01;
const DEC3_MULT: f32 = (1000.0 / 999.0) * 0.001;

/*
const DEC4_MULT: f32 = (10000.0 / 9999.0) * 0.0001;

const OCT1_MULT: f32 = (2.0 / 1.0)   * 0.5;
const OCT2_MULT: f32 = (4.0 / 3.0)   * 0.25;
 */

const OCT3_MULT: f32 = (8.0 / 7.0) * 0.125;
const OCT4_MULT: f32 = (16.0 / 15.0) * 0.0625;

/*
const OCT5_MULT: f32 = (32.0 / 31.0) * 0.03125;
const OCT6_MULT: f32 = (64.0 / 63.0) * 0.015625;
const OCT7_MULT: f32 = (128.0 / 127.0)*0.0078125;
const OCT8_MULT: f32 = (256.0 / 255.0)*0.00390625;
 */

#[inline]
fn safe_positive(v: f32) -> f32 {
    if v.is_finite() && v > 0.0 {
        v
    } else {
        1e-30 // smallest positive to avoid log(0)
    }
}

#[inline]
fn inv_pow_curve(v: f32, mult: f32, base: f32, n: f32) -> f32 {
    // CloudSeed: y = (base^(n*x) - 1) * mult
    // inverse  : x = log_base(y/mult + 1) / n

    let y = v / mult + 1.0;
    let y = safe_positive(y);
    let ln_base = base.ln();

    let x = (y.ln() / ln_base) / n;

    x.clamp(0.0, 1.0)
}

#[inline]
fn inv_resp1dec(v: f32) -> f32 {
    inv_pow_curve(v, DEC1_MULT, 10.0, 1.0)
}

#[inline]
fn inv_resp2dec(v: f32) -> f32 {
    inv_pow_curve(v, DEC2_MULT, 10.0, 2.0)
}

#[inline]
fn inv_resp3dec(v: f32) -> f32 {
    inv_pow_curve(v, DEC3_MULT, 10.0, 3.0)
}

/*
#[inline]
fn inv_resp4dec(v: f32) -> f32 {
    inv_pow_curve(v, DEC4_MULT, 10.0, 4.0)
}

#[inline]
fn inv_resp1oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT1_MULT, 2.0, 1.0)
}

#[inline]
fn inv_resp2oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT2_MULT, 2.0, 2.0)
}
*/

#[inline]
fn inv_resp3oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT3_MULT, 2.0, 3.0)
}
#[inline]
fn inv_resp4oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT4_MULT, 2.0, 4.0)
}

/*
#[inline]
fn inv_resp5oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT5_MULT, 2.0, 5.0)
}

#[inline]
fn inv_resp6oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT6_MULT, 2.0, 6.0)
}

#[inline]
fn inv_resp7oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT7_MULT, 2.0, 7.0)
}

#[inline]
fn inv_resp8oct(v: f32) -> f32 {
    inv_pow_curve(v, OCT8_MULT, 2.0, 8.0)
}
*/
