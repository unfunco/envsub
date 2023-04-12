// Copyright © 2019 Daniel Morris
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::{self, Read};

mod filter;
mod parse;

pub fn envsub(input: &str) -> String {
    let variables = parse::find_variables(input);

    let mut output = input.to_string();
    for (placeholder, name, filters) in variables {
        match std::env::var(&name) {
            Ok(value) => {
                let filtered_value = filter::apply_filters(&value, &filters);
                output = output.replace(&placeholder, &filtered_value);
            },
            Err(_) => {
                if filters.is_empty() || filters[0].0 != "default" {
                    eprintln!(
                        "Warning: Environment variable {} not found.",
                        name
                    );
                } else {
                    let default_value = filter::apply_filters("", &filters);
                    output = output.replace(&placeholder, &default_value);
                }
            },
        }
    }

    output
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = envsub(&input);
    println!("{}", output);

    Ok(())
}
