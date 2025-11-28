use cloudseedcore_rs::{ParamId, ReverbController, DARK_PLATE};

#[test]
fn new_reverb_has_correct_block_size() {
    let r = ReverbController::new(48000.0, 512);
    assert_eq!(r.max_block_size(), 512);
}

#[test]
fn set_and_get_parameter() {
    let mut r = ReverbController::new(48000.0, 64);

    r.set_parameter(ParamId::DryOut, 0.77);
    assert_eq!(r.get_parameter(ParamId::DryOut), 0.77);

    r.set_parameter(ParamId::HighCutEnabled, 1.0);
    assert_eq!(r.get_parameter(ParamId::HighCutEnabled), 1.0);
}

#[test]
fn sample_rate_updates() {
    let mut r = ReverbController::new(44100.0, 64);
    r.set_sample_rate(96000.0);
    // should not panic or crash
}

#[test]
fn reset_does_not_panic() {
    let mut r = ReverbController::new(44100.0, 64);
    r.reset();
}

#[test]
fn program_roundtrip() {
    let mut r = ReverbController::new(48000.0, 64);

    // load DARK_PLATE
    r.set_program(&DARK_PLATE);
    // read back from the C++ engine
    let p = r.get_program();
    // values should match - tests get_all_parameters and load_program
    assert_eq!(p, DARK_PLATE);
}

#[test]
fn program_set_then_modify_parameter() {
    let mut r = ReverbController::new(48000.0, 64);
    r.set_program(&DARK_PLATE);

    r.set_parameter(ParamId::InputMix, 0.123);
    let v = r.get_parameter(ParamId::InputMix);

    assert_eq!(v, 0.123);
}
