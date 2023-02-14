use atty::Stream;
use regex::Regex;
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, BufRead},
};

use argust::ParserConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchMode {
    CompleteMatch,  // c
    SubstringMatch, // s
    FirstSubstring, // f
    LastSubstring,  // l
    AllSubstring,   // a
    PlainSearch,    // Default
}

fn main() {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    let arg_context = argust::parse_args(args.iter(), get_argust_config());

    if arg_context.contains(Some('h'), Some("help")).0 {
        print_help();
        return;
    }

    let lines = get_lines(&arg_context);

    if lines.is_empty() {
        eprintln!("No input recieved.");
        print_help();
        return;
    }

    let match_mode = get_mode(&arg_context);
    if let Some(query) = get_query(&arg_context) {
        let mut query = query;
        query = match match_mode {
            MatchMode::PlainSearch => regex::escape(&query),
            MatchMode::CompleteMatch => format!("^{}$", query),
            _ => query,
        };
        let query = Regex::new(&query).expect("Unable to build the regex query");

        lines
            .iter()
            .filter_map(|l| search(l, &query, match_mode))
            .for_each(|l| println!("{}", l));
    } else {
        print_help();
    }
}

fn get_lines(arg_context: &argust::ArgContext) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    if let Some(file_path) = arg_context.contains(Some('p'), Some("path")).1 {
        lines.append(&mut get_lines_from_file(&file_path));
    } else {
        if !atty::is(Stream::Stdin) {
            let stdin = io::stdin();
            let stdin = stdin.lock();
            for line in stdin.lines() {
                lines.push(line.unwrap());
            }
        }
    }
    return lines;
}

fn get_mode(arg_context: &argust::ArgContext) -> MatchMode {
    if let Some(mode) = arg_context.contains(Some('m'), Some("mode")).1 {
        match mode.as_ref() {
            "c" => return MatchMode::CompleteMatch,
            "s" => return MatchMode::SubstringMatch,
            "f" => return MatchMode::FirstSubstring,
            "l" => return MatchMode::LastSubstring,
            "a" => return MatchMode::AllSubstring,
            _ => return MatchMode::PlainSearch,
        };
    }
    return MatchMode::PlainSearch;
}

fn get_query(arg_context: &argust::ArgContext) -> Option<String> {
    if let Some(query) = arg_context.contains(Some('q'), Some("query")).1 {
        return Some(query);
    }
    return None;
}

fn search(line: &str, query: &Regex, mode: MatchMode) -> Option<String> {
    match mode {
        MatchMode::FirstSubstring => query
            .captures_iter(line)
            .take(1)
            .map(|c| c.get(0).unwrap().as_str().to_string())
            .collect::<Vec<String>>()
            .pop(),
        MatchMode::LastSubstring => query
            .captures_iter(line)
            .last()
            .map(|c| c.get(0).unwrap().as_str().to_string()),
        MatchMode::AllSubstring => query
            .captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().to_string())
            .reduce(|accum, item| (accum + "\n" + &item)),

        _ => {
            if query.is_match(&line) {
                return Some(line.to_string());
            } else {
                return None;
            }
        }
    }
}

fn get_argust_config() -> Option<ParserConfig> {
    let mut parser_config = ParserConfig::new();
    parser_config.parse_tokens.option_key = " ".to_string();
    parser_config.add_parameter('p', "path");
    parser_config.add_parameter('q', "query");
    parser_config.add_parameter('m', "mode");

    return Some(parser_config);
}

fn get_lines_from_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .unwrap_or_else(move |err| panic!("Unable to read from: {}", err))
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn print_help() {
    println!(
        r#"
Command line utility to search and filter strings

Usage:
    rfstr -q <query> [-p <file path>] [-m <mode>]

EXAMPLE:
$ echo "Hello World" | rfstr -q "[[:alpha:]]+" -m f
$ Hello

OPTIONS:

-q, --query     Required    The query that needs to be searched. It can be any 
                            valid rust expression without any named captures.

-p, --path      Optional    The path to file which needs to be searched.

-m, --mode      Optional    The search mode to be used. By default it will be
                            plain text search.

                The available modes are -
                * c - Complete Match      Entire line should match the given regex
                * s - Substring Match,    Print lines that contain the substring matching query
                * f - First Substring,    Print only the first substring of a line that matched
                * l - Last Substring,     Print only the last sustring of a line that matched
                * a - All Substring,      Print all matched substring of a line
                * [Default] PlainSearch,  Print lines containing the subsctring - no regex.    
"#
    );
}
