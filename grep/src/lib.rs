#![deny(warnings)]

use anyhow::{Error, Ok};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    show_line_number: bool,
    list: bool,
    case_insensitive: bool,
    invert: bool,
    entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            show_line_number: flags.iter().any(|&flag| "-n" == flag),
            list: flags.iter().any(|&flag| "-l" == flag),
            case_insensitive: flags.iter().any(|&flag| "-i" == flag),
            invert: flags.iter().any(|&flag| "-v" == flag),
            entire_line: flags.iter().any(|&flag| "-x" == flag),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    // todo!(
    //     "Search the files '{files:?}' for '{pattern}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{flags:?}'"
    // );

    use std::borrow::Cow;
    use std::fs;

    let pattern: Cow<'_, str> = if flags.case_insensitive {
        pattern.to_lowercase().into()
    } else {
        pattern.into()
    };

    let show_file_name = files.len() > 1;

    let mut matches = Vec::new();
    for &file in files {
        let content = fs::read_to_string(file)?;
        let mut file_matched = false;

        for (line_num, line) in content.lines().enumerate() {
            let line_to_match: Cow<'_, str> = if flags.case_insensitive {
                line.to_lowercase().into()
            } else {
                line.into()
            };

            let matched = (flags.entire_line && pattern == line_to_match)
                || (!flags.entire_line && line_to_match.contains(pattern.as_ref()));

            if matched && !flags.invert || !matched && flags.invert {
                file_matched = flags.list;
                if !file_matched {
                    matches.push(format!(
                        "{}{}{}",
                        if show_file_name {
                            Cow::Owned(format!("{}:", file))
                        } else {
                            Cow::Borrowed("")
                        },
                        if flags.show_line_number {
                            Cow::Owned(format!("{}:", line_num + 1))
                        } else {
                            Cow::Borrowed("")
                        },
                        line
                    ));
                }
            }
        }

        if file_matched {
            matches.push(file.to_string());
        }
    }

    Ok(matches)
}
