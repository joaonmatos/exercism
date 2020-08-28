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

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else {
        let screaming = trimmed.to_ascii_uppercase().as_str() == trimmed
            && trimmed.contains(|ch| ch >= 'A' && ch <= 'Z');
        let question = trimmed
            .chars()
            .last()
            .expect("str should not be empty from the guard")
            == '?';
        match (screaming, question) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Whoa, chill out!",
            (false, true) => "Sure.",
            (false, false) => "Whatever.",
        }
    }
}
