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

pub fn apply_filters(
    input: &str,
    filters: &[(String, Option<String>)],
) -> String {
    filters
        .iter()
        .fold(input.to_string(), |input, (filter, arg)| {
            match filter.as_str() {
                "uppercase" => uppercase(&input),
                "lowercase" => lowercase(&input),
                "default" => default(&input, arg.as_deref()),
                _ => input,
            }
        })
}

fn uppercase(input: &str) -> String { input.to_uppercase() }

fn lowercase(input: &str) -> String { input.to_lowercase() }

fn default(input: &str, value: Option<&str>) -> String {
    if input.is_empty() {
        value.unwrap_or("").to_string()
    } else {
        input.to_string()
    }
}
