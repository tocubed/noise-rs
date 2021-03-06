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
use NoiseModule;

/// Noise module that outputs the absolute value of the output value from the
/// source module.
pub struct Abs<Source> {
    /// Outputs a value.
    source: Source,
}

impl<Source> Abs<Source> {
    pub fn new(source: Source) -> Abs<Source> {
        Abs { source: source }
    }
}

impl<Source, T, U> NoiseModule<T> for Abs<Source>
    where Source: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        (self.source.get(point)).abs()
    }
}
