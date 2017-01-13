// Copyright 2015 The Noise-rs Developers.
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
use {NoiseModule, PermutationTable, gradient};

/// Default noise seed for the Perlin noise module.
pub const DEFAULT_PERLIN_SEED: usize = 0;
/// Default period for the Perlin noise module.
pub const DEFAULT_PERLIN_PERIOD: usize = 256;

/// Noise module that outputs 2/3/4-dimensional Perlin noise.
#[derive(Clone, Copy, Debug)]
pub struct Perlin {
    perm_table: PermutationTable,

    /// Seed.
    pub seed: usize,

    /// Extent at which the noise grid wraps around, yielding
    /// seamlessly periodic noise in all dimensions.
    pub period: usize,

    enable_period: bool,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            perm_table: PermutationTable::new(DEFAULT_PERLIN_SEED as u32),
            seed: DEFAULT_PERLIN_SEED,
            period: DEFAULT_PERLIN_PERIOD,
            enable_period: false,
        }
    }

    pub fn set_seed(self, seed: usize) -> Perlin {
        Perlin {
            perm_table: PermutationTable::new(seed as u32),
            seed: seed,
            ..self
        }
    }

    pub fn set_period(self, period: usize) -> Perlin {
        Perlin {
            period: period,
            enable_period: true,
            ..self
        }
    }
}

/// 2-dimensional perlin noise
impl<T: Float> NoiseModule<Point2<T>> for Perlin {
    type Output = T;

    fn get(&self, point: Point2<T>) -> T {
        #[inline(always)]
        fn surflet<T: Float>(perm_table: &PermutationTable,
                             corner: math::Point2<isize>,
                             distance: math::Vector2<T>)
                             -> T {
            let attn = T::one() - math::dot2(distance, distance);
            if attn > T::zero() {
                math::pow4(attn) * math::dot2(distance, gradient::get2(perm_table.get2(corner)))
            } else {
                T::zero()
            }
        }

        let floored = math::map2(point, T::floor);
        let near_distance = math::sub2(point, floored);
        let far_distance = math::sub2(near_distance, math::one2());

        let (near_corner, far_corner) = if self.enable_period {
            let near = math::map2(floored, math::cast);
            let near = math::mod2(near, math::cast(self.period));
            let far = math::add2(near, math::one2());
            let far = math::mod2(far, math::cast(self.period));
            (near, far)
        } else {
            let near = math::map2(floored, math::cast);
            let far = math::add2(near, math::one2());
            (near, far)
        };

        let f00 = surflet(&self.perm_table,
                          [near_corner[0], near_corner[1]],
                          [near_distance[0], near_distance[1]]);
        let f10 = surflet(&self.perm_table,
                          [far_corner[0], near_corner[1]],
                          [far_distance[0], near_distance[1]]);
        let f01 = surflet(&self.perm_table,
                          [near_corner[0], far_corner[1]],
                          [near_distance[0], far_distance[1]]);
        let f11 = surflet(&self.perm_table,
                          [far_corner[0], far_corner[1]],
                          [far_distance[0], far_distance[1]]);

        // Multiply by arbitrary value to scale to -1..1
        (f00 + f10 + f01 + f11) * math::cast(3.1604938271604937)
    }
}

/// 3-dimensional perlin noise
impl<T: Float> NoiseModule<Point3<T>> for Perlin {
    type Output = T;

    fn get(&self, point: Point3<T>) -> T {
        #[inline(always)]
        fn surflet<T: Float>(perm_table: &PermutationTable,
                             corner: math::Point3<isize>,
                             distance: math::Vector3<T>)
                             -> T {
            let attn = T::one() - math::dot3(distance, distance);
            if attn > T::zero() {
                math::pow4(attn) * math::dot3(distance, gradient::get3(perm_table.get3(corner)))
            } else {
                T::zero()
            }
        }

        let floored = math::map3(point, T::floor);
        let near_distance = math::sub3(point, floored);
        let far_distance = math::sub3(near_distance, math::one3());

        let (near_corner, far_corner) = if self.enable_period {
            let near = math::map3(floored, math::cast);
            let near = math::mod3(near, math::cast(self.period));
            let far = math::add3(near, math::one3());
            let far = math::mod3(far, math::cast(self.period));
            (near, far)
        } else {
            let near = math::map3(floored, math::cast);
            let far = math::add3(near, math::one3());
            (near, far)
        };

        let f000 = surflet(&self.perm_table,
                           [near_corner[0], near_corner[1], near_corner[2]],
                           [near_distance[0], near_distance[1], near_distance[2]]);
        let f100 = surflet(&self.perm_table,
                           [far_corner[0], near_corner[1], near_corner[2]],
                           [far_distance[0], near_distance[1], near_distance[2]]);
        let f010 = surflet(&self.perm_table,
                           [near_corner[0], far_corner[1], near_corner[2]],
                           [near_distance[0], far_distance[1], near_distance[2]]);
        let f110 = surflet(&self.perm_table,
                           [far_corner[0], far_corner[1], near_corner[2]],
                           [far_distance[0], far_distance[1], near_distance[2]]);
        let f001 = surflet(&self.perm_table,
                           [near_corner[0], near_corner[1], far_corner[2]],
                           [near_distance[0], near_distance[1], far_distance[2]]);
        let f101 = surflet(&self.perm_table,
                           [far_corner[0], near_corner[1], far_corner[2]],
                           [far_distance[0], near_distance[1], far_distance[2]]);
        let f011 = surflet(&self.perm_table,
                           [near_corner[0], far_corner[1], far_corner[2]],
                           [near_distance[0], far_distance[1], far_distance[2]]);
        let f111 = surflet(&self.perm_table,
                           [far_corner[0], far_corner[1], far_corner[2]],
                           [far_distance[0], far_distance[1], far_distance[2]]);

        // Multiply by arbitrary value to scale to -1..1
        (f000 + f100 + f010 + f110 + f001 + f101 + f011 + f111) * math::cast(3.8898553255531074)
    }
}

/// 4-dimensional perlin noise
impl<T: Float> NoiseModule<Point4<T>> for Perlin {
    type Output = T;

    fn get(&self, point: Point4<T>) -> T {
        #[inline(always)]
        fn surflet<T: Float>(perm_table: &PermutationTable,
                             corner: math::Point4<isize>,
                             distance: math::Vector4<T>)
                             -> T {
            let attn = T::one() - math::dot4(distance, distance);
            if attn > T::zero() {
                math::pow4(attn) * math::dot4(distance, gradient::get4(perm_table.get4(corner)))
            } else {
                T::zero()
            }
        }

        let floored = math::map4(point, T::floor);
        let near_distance = math::sub4(point, floored);
        let far_distance = math::sub4(near_distance, math::one4());

        let (near_corner, far_corner) = if self.enable_period {
            let near = math::map4(floored, math::cast);
            let near = math::mod4(near, math::cast(self.period));
            let far = math::add4(near, math::one4());
            let far = math::mod4(far, math::cast(self.period));
            (near, far)
        } else {
            let near = math::map4(floored, math::cast);
            let far = math::add4(near, math::one4());
            (near, far)
        };

        let f0000 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                    [near_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
        let f1000 =
            surflet(&self.perm_table,
                    [far_corner[0], near_corner[1], near_corner[2], near_corner[3]],
                    [far_distance[0], near_distance[1], near_distance[2], near_distance[3]]);
        let f0100 =
            surflet(&self.perm_table,
                    [near_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                    [near_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
        let f1100 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], near_corner[2], near_corner[3]],
                            [far_distance[0], far_distance[1], near_distance[2], near_distance[3]]);
        let f0010 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                    [near_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
        let f1010 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], far_corner[2], near_corner[3]],
                            [far_distance[0], near_distance[1], far_distance[2], near_distance[3]]);
        let f0110 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                            [near_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
        let f1110 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], far_corner[2], near_corner[3]],
                            [far_distance[0], far_distance[1], far_distance[2], near_distance[3]]);
        let f0001 =
            surflet(&self.perm_table,
                    [near_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                    [near_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
        let f1001 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], near_corner[2], far_corner[3]],
                            [far_distance[0], near_distance[1], near_distance[2], far_distance[3]]);
        let f0101 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                            [near_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
        let f1101 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], near_corner[2], far_corner[3]],
                            [far_distance[0], far_distance[1], near_distance[2], far_distance[3]]);
        let f0011 = surflet(&self.perm_table,
                            [near_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                            [near_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
        let f1011 = surflet(&self.perm_table,
                            [far_corner[0], near_corner[1], far_corner[2], far_corner[3]],
                            [far_distance[0], near_distance[1], far_distance[2], far_distance[3]]);
        let f0111 = surflet(&self.perm_table,
                            [near_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                            [near_distance[0], far_distance[1], far_distance[2], far_distance[3]]);
        let f1111 = surflet(&self.perm_table,
                            [far_corner[0], far_corner[1], far_corner[2], far_corner[3]],
                            [far_distance[0], far_distance[1], far_distance[2], far_distance[3]]);

        // Multiply by arbitrary value to scale to -1..1
        (f0000 + f1000 + f0100 + f1100 + f0010 + f1010 + f0110 + f1110 + f0001 +
         f1001 + f0101 + f1101 + f0011 + f1011 + f0111 + f1111) *
        math::cast(4.424369240215691)
    }
}
