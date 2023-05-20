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

/// Filter type representing a filter, and an optional argument.
/// At the moment we only have a three filters, which is pretty useless,
/// and only one of them accepts an argument, so this type will need to
/// be expanded upon in the future to potentially accept additional
/// arguments.
type Filter = (String, Option<String>);

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
