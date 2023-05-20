// Copyright © 2023 Daniel Morris
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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VARIABLE_REGEX: Regex = Regex::new(r"\$\{(.+?)\}").unwrap();
    static ref FILTER_REGEX: Regex =
        Regex::new(r"([^()\s]+)\(([^()]*)\)").unwrap();
}

type Placeholder = Vec<(String, String, Vec<(String, Option<String>)>)>;

pub fn find_variables(input: &str) -> Placeholder {
    VARIABLE_REGEX
        .captures_iter(input)
        .map(|v| {
            let placeholder = v[0].to_string();
            let contents = v[1].trim();
            let mut parts = contents.split('|').map(|s| s.trim().to_string());

            let name = parts.next().unwrap_or_default();
            let filters = parts.map(parse_filter).collect();

            (placeholder, name, filters)
        })
        .collect()
}

fn parse_filter(input: String) -> (String, Option<String>) {
    if let Some(filter) = FILTER_REGEX.captures(&input) {
        let name = filter[1].to_string();
        let arg = Some(filter[2].to_string().trim().replace('\"', ""));
        (name, arg)
    } else {
        (input, None)
    }
}
