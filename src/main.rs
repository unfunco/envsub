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

use nom::bytes::complete::{tag, take_till};
use nom::sequence::{delimited, pair};
use nom::IResult;
use std::env;
use std::io::{self, Read};

fn parse(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(tag("${"), pair(take_till(|c| c == '}'), tag("}")), tag("}"))(input)
}

fn envsub(contents: &str) -> String {
    let mut result = String::new();
    let mut remaining = contents;

    while let Ok((next, (before_var, var_name))) = parse(remaining) {
        result.push_str(before_var);

        if let Ok(var_value) = env::var(var_name) {
            result.push_str(&var_value);
        } else {
            eprintln!("Warning: environment variable {} not found", var_name);
        }

        remaining = next;
    }

    result.push_str(remaining);
    result
}

fn main() -> io::Result<()> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;

    let output = envsub(&contents);
    println!("{}", output);

    Ok(())
}
