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

pub fn is_armstrong_number(num: u32) -> bool {
    let len = no_digits_u32(num);
    let mut divider = 1;
    let mut acc = 0;
    for _ in 0..len {
        let digit = (num / divider) % 10;
        acc += digit.pow(len);
        divider *= 10;
    }
    acc == num
}

fn no_digits_u32(num: u32) -> u32 {
    match num {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        100000..=999999 => 6,
        1000000..=9999999 => 7,
        10000000..=99999999 => 8,
        100000000..=999999999 => 9,
        _ => 10,
    }
}
