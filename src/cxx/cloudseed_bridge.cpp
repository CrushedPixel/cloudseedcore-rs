#include "cloudseed_bridge.h"
#include <memory>
#include <algorithm>
#include "cloudseed_shims.h"

// CloudSeedCore headers.
// CloudSeedCore is added to the include path in build.rs
#include "DSP/ReverbController.h"
#include "Parameters.h"

using Cloudseed::ReverbController;

CloudSeedReverb::CloudSeedReverb(float sample_rate, uint32_t max_block)
    : controller(new ReverbController(static_cast<int>(sample_rate))),
      sample_rate(sample_rate), max_block(max_block) {}

CloudSeedReverb::~CloudSeedReverb() {
    delete controller;
}

void CloudSeedReverb::reset() {
    controller->ClearBuffers();
}

void CloudSeedReverb::set_sample_rate(float sample_rate) {
    this->sample_rate = sample_rate;
    controller->SetSamplerate(static_cast<int>(sample_rate));
}

void CloudSeedReverb::set_parameter(uint32_t id, float value) {
    int pid = static_cast<int>(id);
    if (pid >= 0 && pid < Cloudseed::Parameter::COUNT) {
        controller->SetParameter(pid, static_cast<double>(value));
    }
}

float CloudSeedReverb::get_parameter(uint32_t id) const {
    return static_cast<float>(controller->GetAllParameters()[id]);
}

void CloudSeedReverb::get_all_parameters(rust::Slice<float> out) const {
    int count = Cloudseed::Parameter::COUNT;
    int n = std::min(static_cast<int>(out.size()), count);
    double* src = controller->GetAllParameters();
    for (int i = 0; i < n; ++i) {
        out[i] = static_cast<float>(src[i]);
    }
}

void CloudSeedReverb::load_program(rust::Slice<const float> params) {
    int count = Cloudseed::Parameter::COUNT;

    // ensure all parameters are provided
    if (static_cast<int>(params.size()) != count) {
        return;
    }

    for (int pid = 0; pid < count; ++pid) {
        controller->SetParameter(pid, static_cast<double>(params[pid]));
    }

    // re-apply configuration by resetting sample rate to trigger UpdateLines, then clear buffers
    controller->SetSamplerate(static_cast<int>(sample_rate));
    controller->ClearBuffers();
}

void CloudSeedReverb::process(rust::Slice<const float> in_l,
                              rust::Slice<const float> in_r,
                              rust::Slice<float> out_l,
                              rust::Slice<float> out_r,
                              uint32_t frames) {
    // CloudSeedCore expects non-const float* for the input channels,
    // but never actually modifies them. therefore, this const_cast is safe.
    float* inL = const_cast<float*>(in_l.data());
    float* inR = const_cast<float*>(in_r.data());
    float* outL = out_l.data();
    float* outR = out_r.data();

    int n = static_cast<int>(frames);
    controller->Process(inL, inR, outL, outR, n);
}

rust::String format_parameter(uint32_t param_id, float value) {
    char buffer[MAX_STR_SIZE];
    Cloudseed::FormatParameter(value, MAX_STR_SIZE, static_cast<int>(param_id), buffer);
    return rust::String(buffer);
}
