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

pub fn square_of_sum(n: u32) -> u32 {
    // based on the sum of arithmetic series formula
    // function implemented below for clarity
    let sum = n * (1 + n) / 2;
    sum * sum
}

fn _arithmetic_series(first_term: u32, number_of_terms: u32, step: u32) -> u32 {
    let last_term = first_term + step * (number_of_terms - 1);
    number_of_terms * (first_term + last_term) / 2
}

pub fn sum_of_squares(n: u32) -> u32 {
    // using Gauss's formula. see: https://brilliant.org/wiki/sum-of-n-n2-or-n3/
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
