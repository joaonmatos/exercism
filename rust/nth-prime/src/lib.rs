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

// There's probably a more efficient way of getting this problem done without
// using a sieve, but I wanted to take the approach and apply a comical ammount
// of optimisations. With that in mind, I joined the following approaches:
//
//  1. Look-up table: the first 8 primes are precomputed;
//  2. Only lookup numbers which are congruent with 1 or 5 mod 6. The others are
//     either divisible by 2 or 3 or both;
//  3. Memoise all the primes that are found along the way. This adds a space
//     complexity of O(n) and a maximum memory use of just over 16GB, but allows
//     us to have much shorter iterations, because we do not uselessly use complex
//     numbers for primality checks. This allows us to somewhat compensate the fact
//     that the spacing between primes tends to become bigger, although there is
//     still a candidate for every 2 in 6 numbers until n primes are found.

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 11,
        5 => 13,
        6 => 17,
        7 => 19,
        _ => sieve(n),
    }
}

fn sieve(n: u32) -> u32 {
    let mut primes = Vec::<u32>::with_capacity(n as usize - 1);
    primes.extend([5, 7, 11, 13, 17, 19].iter());
    for candidate in SixMultiple::new() {
        if primes.len() == n as usize - 1 {
            break;
        }
        let mut prime = true;
        for factor in &primes {
            if factor * factor > candidate {
                break;
            }
            if candidate % factor == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(candidate);
        }
    }
    primes.pop().unwrap()
}

struct SixMultiple {
    state: SMState,
}

impl SixMultiple {
    fn new() -> Self {
        Self {
            state: SMState::Minus(23),
        }
    }
}

impl Iterator for SixMultiple {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.state {
            SMState::Minus(n) => n,
            SMState::Plus(n) => n,
        };
        self.state = match self.state {
            SMState::Minus(n) => SMState::Plus(n + 2),
            SMState::Plus(n) => SMState::Minus(n + 4),
        };
        Some(out)
    }
}

enum SMState {
    Minus(u32),
    Plus(u32),
}
