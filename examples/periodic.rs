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

extern crate noise;

use noise::modules::{Add, Perlin, RidgedMulti, Turbulence};

mod debug;

fn main() {
    let ridged = RidgedMulti::new().set_period(7);
    let turbulence = Turbulence::new(ridged).set_period(7);

    let perlin = Perlin::new().set_period(7);

    let result = Add::new(turbulence, perlin);

    debug::render_png2("periodic.png", result, 1024, 1024, 50);
}
