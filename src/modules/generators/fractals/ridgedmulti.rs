// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::Float;
use math;
use math::{Point2, Point3, Point4};
use NoiseModule;
use modules::Perlin;

/// Default noise seed for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_SEED: usize = 0;
/// Default number of octaves for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_OCTAVE_COUNT: usize = 6;
/// Default frequency for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_FREQUENCY: f32 = 1.0;
/// Default lacunarity for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_LACUNARITY: f32 = 2.0;
/// Default persistence for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_PERSISTENCE: f32 = 1.0;
/// Default gain for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_GAIN: f32 = 2.0;
/// Default period for the RidgedMulti noise module.
pub const DEFAULT_RIDGED_PERIOD: usize = 256;
/// Maximum number of octaves for the RidgedMulti noise module.
pub const RIDGED_MAX_OCTAVES: usize = 32;

/// Noise module that outputs ridged-multifractal noise.
///
/// This noise module, heavily based on the fBm-noise module, generates
/// ridged-multifractal noise. Ridged-multifractal noise is generated in much
/// the same way as fBm noise, except the output of each octave is modified by
/// an absolute-value function. Modifying the octave values in this way
/// produces ridge-like formations.
///
/// The values output from this module will usually range from -1.0 to 1.0 with
/// default values for the parameters, but there are no guarantees that all
/// output values will exist within this range. If the parameters are modified
/// from their defaults, then the output will need to be scaled to remain in
/// the [-1,1] range.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
#[derive(Clone, Debug)]
pub struct RidgedMulti<T> {
    /// Seed.
    pub seed: usize,

    /// Total number of frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: usize,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: T,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: T,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: T,

    /// The gain to apply to the weight on each octave.
    pub gain: T,

    /// Extent at which the noise grid wraps around, yielding
    /// seamlessly periodic noise in all dimensions.
    pub period: usize,

    enable_period: bool,

    sources: Vec<Perlin>,
}

impl<T: Float> RidgedMulti<T> {
    pub fn new() -> RidgedMulti<T> {
        RidgedMulti {
            seed: DEFAULT_RIDGED_SEED,
            octaves: DEFAULT_RIDGED_OCTAVE_COUNT,
            frequency: math::cast(DEFAULT_RIDGED_FREQUENCY),
            lacunarity: math::cast(DEFAULT_RIDGED_LACUNARITY),
            persistence: math::cast(DEFAULT_RIDGED_PERSISTENCE),
            gain: math::cast(DEFAULT_RIDGED_GAIN),
            period: DEFAULT_RIDGED_PERIOD,
            enable_period: false,
            sources: super::build_sources(DEFAULT_RIDGED_SEED, DEFAULT_RIDGED_OCTAVE_COUNT),
        }
    }

    pub fn set_seed(self, seed: usize) -> RidgedMulti<T> {
        if self.seed == seed {
            return self;
        }
        if !self.enable_period {
            RidgedMulti {
                seed: seed,
                sources: super::build_sources(seed, self.octaves),
                ..self
            }
        } else {
            RidgedMulti {
                seed: seed,
                sources: super::build_sources_periodic(seed, self.octaves, self.period, self.lacunarity),
                ..self
            }
        }
    }

    pub fn set_octaves(self, mut octaves: usize) -> RidgedMulti<T> {
        if self.octaves == octaves {
            return self;
        } else if octaves > RIDGED_MAX_OCTAVES {
            octaves = RIDGED_MAX_OCTAVES;
        } else if octaves < 1 {
            octaves = 1;
        }
        if !self.enable_period {
            RidgedMulti {
                octaves: octaves,
                sources: super::build_sources(self.seed, octaves),
                ..self
            }
        } else {
            RidgedMulti {
                octaves: octaves,
                sources: super::build_sources_periodic(self.seed, octaves, self.period, self.lacunarity),
                ..self
            }
        }
    }

    pub fn set_frequency(self, frequency: T) -> RidgedMulti<T> {
        RidgedMulti { frequency: frequency, ..self }
    }

    pub fn set_lacunarity(self, lacunarity: T) -> RidgedMulti<T> {
        if !self.enable_period {
            RidgedMulti { lacunarity: lacunarity, ..self }
        } else {
            RidgedMulti {
                lacunarity: lacunarity,
                sources: super::build_sources_periodic(self.seed, self.octaves, self.period, lacunarity),
                ..self
            }
        }
    }

    pub fn set_period(self, period: usize) -> RidgedMulti<T> {
        RidgedMulti {
            period: period,
            enable_period: true,
            sources: super::build_sources_periodic(self.seed, self.octaves, period, self.lacunarity),
            ..self
        }
    }

    pub fn set_persistence(self, persistence: T) -> RidgedMulti<T> {
        RidgedMulti { persistence: persistence, ..self }
    }

    pub fn set_gain(self, gain: T) -> RidgedMulti<T> {
        RidgedMulti { gain: gain, ..self }
    }
}

/// 2-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point2<T>> for RidgedMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point2<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul2(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight succesive contributions by the previous signal.
            weight = signal * self.gain;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul2(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        result.mul_add(math::cast(1.0 / 3.0), -T::one())
    }
}

/// 3-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point3<T>> for RidgedMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point3<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul3(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight succesive contributions by the previous signal.
            weight = signal * self.gain;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul3(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        result.mul_add(math::cast(1.0 / 3.0), -T::one())
    }
}

/// 4-dimensional RidgedMulti noise
impl<T: Float> NoiseModule<Point4<T>> for RidgedMulti<T> {
    type Output = T;

    fn get(&self, mut point: Point4<T>) -> T {
        let mut result = T::zero();
        let mut weight = T::one();

        point = math::mul4(point, self.frequency);

        for x in 0..self.octaves {
            // Get the value.
            let mut signal = self.sources[x].get(point);

            // Make the ridges.
            signal = signal.abs();
            signal = T::one() - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal = signal * signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal = signal * weight;

            // Weight succesive contributions by the previous signal.
            weight = signal * self.gain;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            if math::cast::<_, f32>(weight) > 1.0 {
                weight = T::one();
            } else if math::cast::<_, f32>(weight) < 0.0 {
                weight = T::zero();
            }

            // Scale the amplitude appropriately for this frequency.
            signal = signal * self.persistence.powi(math::cast(x));

            // Add the signal to the result.
            result = result + signal;

            // Increase the frequency.
            point = math::mul4(point, self.lacunarity);
        }

        // Scale and shift the result into the [-1,1] range
        result.mul_add(math::cast(1.0 / 3.0), -T::one())
    }
}
