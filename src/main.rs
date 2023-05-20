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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PLACEHOLDER_RE: Regex = Regex::new(r"\$\{(.+?)\}").unwrap();
    static ref FILTER_RE: Regex =
        Regex::new(r"([^()\s]+)\(([^()]*)\)").unwrap();
}

// OptionalArg represents an optional argument.
type OptionalArg = Option<String>;

// Filter represents a filter function and its optional argument.
type Filter = (String, OptionalArg);

// Placeholder represents a placeholder in the input, which includes
// the placeholder string, the environment variable name, and any filters
// applied.
type Placeholder = (String, String, Vec<Filter>);

// envsub replaces placeholders in the input string with environment variables,
// applying any filters that are included with the placeholders.
pub fn envsub(input: &str) -> String {
    let placeholders = extract_placeholders(input);
    let mut output = input.to_string();
    for (placeholder, name, filters) in placeholders {
        match std::env::var(&name) {
            Ok(value) => {
                let filtered_value = apply_filters(&value, &filters);
                output = output.replace(&placeholder, &filtered_value);
            },
            Err(_) => {
                if filters.is_empty() || filters[0].0 != "default" {
                    eprintln!(
                        "Warning: Environment variable {} not found.",
                        name
                    );
                } else {
                    let default_value = apply_filters("", &filters);
                    output = output.replace(&placeholder, &default_value);
                }
            },
        }
    }

    output
}

// extract_placeholders parses the input string and extracts all placeholders.
pub fn extract_placeholders(input: &str) -> Vec<Placeholder> {
    PLACEHOLDER_RE
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

// parse_filter parses a filter function and its optional argument from a
// string.
fn parse_filter(input: String) -> Filter {
    if let Some(filter) = FILTER_RE.captures(&input) {
        let name = filter[1].to_string();
        let arg0 = Some(filter[2].to_string().trim().replace('\"', ""));
        (name, arg0)
    } else {
        (input, None)
    }
}

// apply_filters applies a sequence of filters to a value.
pub fn apply_filters(value: &str, filters: &[Filter]) -> String {
    filters
        .iter()
        .fold(value.to_string(), |value, (filter, arg0)| {
            match filter.as_str() {
                "uppercase" => uppercase(&value),
                "lowercase" => lowercase(&value),
                "default" => default(&value, arg0.as_deref()),
                "trim" => trim(&value),
                _ => value,
            }
        })
}

fn uppercase(value: &str) -> String { value.to_uppercase() }
fn lowercase(value: &str) -> String { value.to_lowercase() }
fn trim(value: &str) -> String { value.trim().to_string() }
fn default(value: &str, default_value: Option<&str>) -> String {
    if value.is_empty() {
        default_value.unwrap_or("").to_string()
    } else {
        value.to_string()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = envsub(&input);
    print!("{}", output);

    Ok(())
}
