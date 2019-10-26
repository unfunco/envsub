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
//
//! Crate `envsub` provides environment variable substitution with Bash-esque
//! variable expansion.

/// Parses the input string and returns the parsed string.
pub fn parse(input: &str) -> String {
    let output = String::from(input);
    output
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("Hello, world!", parse("Hello, world!"));
    }
}
