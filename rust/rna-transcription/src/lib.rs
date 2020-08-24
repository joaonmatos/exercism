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

#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let bad_char = dna
            .chars()
            .position(|c| !(c == 'A' || c == 'C' || c == 'G' || c == 'T'));
        match bad_char {
            Some(index) => Err(index),
            None => Ok(Self {
                strand: String::from(dna),
            }),
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            strand: self.strand.chars().map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!()
            }).collect()
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let bad_char = rna
            .chars()
            .position(|c| !(c == 'A' || c == 'C' || c == 'G' || c == 'U'));
        match bad_char {
            Some(index) => Err(index),
            None => Ok(Self {
                strand: String::from(rna),
            }),
        }
    }
}
