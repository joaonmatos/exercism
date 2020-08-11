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

pub fn verse(n: u32) -> String {
    let first_part = format!(
        "{} of beer on the wall, {} of beer.",
        format_number_bottles(n),
        format_number_bottles(n).to_ascii_lowercase()
    );
    let second_part = if n == 0 {
        String::from("Go to the store and buy some more, 99 bottles of beer on the wall.")
    } else {
        format!(
            "Take {} down and pass it around, {} of beer on the wall.",
            if n == 1 { "it" } else { "one" },
            format_number_bottles(n - 1).to_ascii_lowercase()
        )
    };
    format!("{}\n{}\n", first_part, second_part)
}

fn format_number_bottles(n: u32) -> String {
    format!(
        "{} {}",
        if n == 0 {
            String::from("No more")
        } else {
            n.to_string()
        },
        if n == 1 {
            String::from("bottle")
        } else {
            String::from("bottles")
        }
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut acc = String::new();
    for n in (end..=start).rev() {
        acc.push_str(verse(n).as_str());
        acc.push('\n');
    }
    acc.pop();
    acc
}
