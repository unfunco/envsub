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
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::env;
use std::io::{self, Read};

fn parse(input: &str) -> IResult<&str, (&str, &str)> {
    let (remaining, (before, _, name)) = tuple((
        take_till(|c| c == '$'),
        tag("$"),
        delimited(tag("{"), take_till(|c| c == '}'), tag("}")),
    ))(input)?;

    Ok((remaining, (before, name)))
}

fn envsub(contents: &str) -> String {
    let mut output = String::new();
    let mut remaining = contents;

    while let Ok((next, (before, name))) = parse(remaining) {
        output.push_str(before);

        if let Ok(value) = env::var(name) {
            output.push_str(&value);
        } else {
            eprintln!("Warning: environment variable {} not found", name);
        }

        remaining = next;
    }

    output.push_str(remaining);
    output
}

fn main() -> io::Result<()> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;

    let output = envsub(&contents);
    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::envsub;
    use std::env;

    #[test]
    fn test_no_variables() {
        let input = "Hello, world!";
        let output = "Hello, world!";
        assert_eq!(envsub(input), output);
    }

    #[test]
    fn test_single_variable() {
        env::set_var("NAME", "Daniel");
        let input = "Hello, ${NAME}!";
        let output = "Hello, Daniel!";
        assert_eq!(envsub(input), output);
    }

    #[test]
    fn test_multiple_variables() {
        env::set_var("FORENAME", "Daniel");
        env::set_var("SURNAME", "Morris");
        let input = "Hello ${FORENAME} ${SURNAME}";
        let output = "Hello Daniel Morris";
        assert_eq!(envsub(input), output);
    }
}
