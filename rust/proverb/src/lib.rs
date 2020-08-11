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

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut acc = String::new();
    let mut it = list.iter().peekable();
    while let Some(word) = it.next() {
        if let Some(next) = it.peek() {
            acc.push_str(format!("For want of a {} the {} was lost.\n", word, next).as_str())
        }
    }
    format!(
        "{}And all for the want of a {}.",
        acc,
        list.first().unwrap()
    )
}
