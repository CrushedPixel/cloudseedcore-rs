use cloudseedcore_rs::{format_parameter_value, LateMode, ParamId, Program, DARK_PLATE};

#[test]
fn program_roundtrip_to_array() {
    let original = DARK_PLATE;

    let arr = original.to_array();
    assert_eq!(arr.len(), 45);

    let reconstructed = Program::from_array(arr);
    assert_eq!(reconstructed, original);
}

#[test]
fn program_from_slice_valid() {
    let p = DARK_PLATE;
    let slice = p.to_array().to_vec();

    let loaded = Program::from_slice(&slice);
    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap(), p);
}

#[test]
fn program_from_slice_invalid_len() {
    let slice = vec![0.0_f32; 44];
    assert!(Program::from_slice(&slice).is_none());
}

#[test]
fn program_get_set_works() {
    let mut p = DARK_PLATE;

    p.set(ParamId::DryOut, 0.42);
    assert!((p.get(ParamId::DryOut) - 0.42).abs() < 1e-6);

    p.set(ParamId::Interpolation, 0.0);
    assert!(!p.interpolation);
    assert_eq!(p.get(ParamId::Interpolation), 0.0);

    p.set(ParamId::Interpolation, 1.0);
    assert!(p.interpolation);
    assert_eq!(p.get(ParamId::Interpolation), 1.0);
}

#[test]
fn late_mode_roundtrip() {
    let mut p = DARK_PLATE;

    p.set(ParamId::LateMode, 0.0);
    assert_eq!(p.late_mode, LateMode::Pre);
    assert_eq!(p.get(ParamId::LateMode), 0.0);

    p.set(ParamId::LateMode, 1.0);
    assert_eq!(p.late_mode, LateMode::Post);
    assert_eq!(p.get(ParamId::LateMode), 1.0);
}

#[test]
fn format_parameter_value_works() {
    let s = format_parameter_value(ParamId::InputMix, 0.6);
    assert_eq!(s, "60%");
}
