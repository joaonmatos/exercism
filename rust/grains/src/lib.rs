// Copyright 2020 João Nuno Matos
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

pub fn square(s: u32) -> u64 {
    // if we double the grains on every square then we have an exponential function
    // in this case it is 2^(n-1). powers or two can be done fastest with bitwise shifts
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    // the sequence of powers of the same ratio is called an arithmetic series
    // according to https://en.wikipedia.org/wiki/Geometric_series the sum is
    // 1 * (1 - 2^64) / (1 - 2) = 2^64 - 1, which is curiously the biggest u64 possible
    std::u64::MAX
}
