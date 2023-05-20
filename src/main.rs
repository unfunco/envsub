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
    /// Regex to match a variable placeholder, which is a variable name
    /// enclosed in "${}".
    static ref PLACEHOLDER_RE: Regex = Regex::new(r"\$\{(.+?)\}").unwrap();
    /// Regex to match a filter, which is a function name followed by optional
    /// arguments in "()".
    static ref FILTER_RE: Regex =
        Regex::new(r"([^()\s]+)\(([^()]*)\)").unwrap();
}

/// Filter type representing a filter, and an optional argument.
/// At the moment we only have a three filters, which is pretty useless,
/// and only one of them accepts an argument, so this type will need to
/// be expanded upon in the future to potentially accept additional
/// arguments.
type Filter = (String, Option<String>);

/// Type representing a placeholder, which is a tuple consisting of the entire
/// placeholder string, the name of an environment variable, and a slice of
/// Filters.
type Placeholder = (String, String, Vec<Filter>);

/// Apply the given filters to the given value.
/// This functions takes a string value and a slice of filters, each filter
/// represented in a tuple of a filter name and optional argument. It applies
/// each filter to the value in order, passing the result of each operation to
/// the next filter.
pub fn apply_filters(value: &str, filters: &[Filter]) -> String {
    filters.iter().fold(value.to_string(), |v, (filter, a0)| {
        match filter.as_str() {
            "uppercase" => uppercase(&v),
            "lowercase" => lowercase(&v),
            "default" => default(&v, a0.as_deref()),
            _ => v,
        }
    })
}

/// Transform the given value to uppercase.
fn uppercase(value: &str) -> String { value.to_uppercase() }

/// Transform the given value to lowercase.
fn lowercase(value: &str) -> String { value.to_lowercase() }

/// Replace an empty value with a default value.
fn default(value: &str, default_value: Option<&str>) -> String {
    if value.is_empty() {
        default_value.unwrap_or("").to_string()
    } else {
        value.to_string()
    }
}

/// Extract all placeholders from the given input string.
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

/// Parse a filter from the given input string.
///
/// This function looks for a filter function and its optional argument in the
/// input string. The filter is expected to be in the form of
/// "<filter>(<argument>)".
fn parse_filter(input: String) -> (String, Option<String>) {
    if let Some(filter) = FILTER_RE.captures(&input) {
        let name = filter[1].to_string();
        let a0 = Some(filter[2].to_string().trim().replace('\"', ""));
        (name, a0)
    } else {
        (input, None)
    }
}

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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = envsub(&input);
    println!("{}", output);

    Ok(())
}
