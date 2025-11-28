fn main() {
    // build the cxx bridge
    let mut bridge = cxx_build::bridge("src/bridge.rs");

    bridge
        .flag_if_supported("-std=c++14")
        .flag_if_supported("/std:c++14")
        .file("src/cxx/cloudseed_bridge.cpp")
        .include("src/cxx")
        .include("vendor/CloudSeedCore")
        // Preprocessor definitions expected by CloudSeedCore
        .define("BUFFER_SIZE", Some("1024"))
        .define("MAX_STR_SIZE", Some("32"))
        // CloudSeedCore cpp files
        .file("vendor/CloudSeedCore/Parameters.cpp")
        .file("vendor/CloudSeedCore/DSP/Biquad.cpp")
        .file("vendor/CloudSeedCore/DSP/RandomBuffer.cpp")
        // include shims for cross-platform compatibility
        .flag("-include")
        .flag("src/cxx/cloudseed_shims.h");

    bridge.compile("cloudseed_core_cxx");

    // invalidate the build if the wrapper or headers change
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/cxx/cloudseed_bridge.h");
    println!("cargo:rerun-if-changed=src/cxx/cloudseed_bridge.cpp");
    println!("cargo:rerun-if-changed=CloudSeedCore");
}
