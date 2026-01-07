#pragma once

#include <cstdint>
#include <cstddef>
#include <memory>
#include "rust/cxx.h"

namespace Cloudseed { class ReverbController; }

// Wrapper class for ReverbController's ReverbController, exposed to Rust via cxx
class CloudSeedReverb {
public:
    CloudSeedReverb(float sample_rate, uint32_t max_block);
    ~CloudSeedReverb();

    void reset();
    void set_sample_rate(float sample_rate);
    float get_sample_rate() const;

    void set_parameter(uint32_t id, float value);
    float get_parameter(uint32_t id) const;
    void get_all_parameters(rust::Slice<float> out) const;

    void load_program(rust::Slice<const float> params);

    void process(rust::Slice<const float> in_l,
                 rust::Slice<const float> in_r,
                 rust::Slice<float> out_l,
                 rust::Slice<float> out_r,
                 uint32_t frames);

private:
    /** The underlying reverb controller. */
    Cloudseed::ReverbController* controller = nullptr;

    float sample_rate = 48000.0f;
    uint32_t max_block = 0;
};

inline std::unique_ptr<CloudSeedReverb> cs_new_reverb(float sample_rate, uint32_t max_block) {
    return std::unique_ptr<CloudSeedReverb>(new CloudSeedReverb(sample_rate, max_block));
}

/** Formats a normalized parameter value to text. */
rust::String cs_format_parameter(uint32_t param_id, float value);