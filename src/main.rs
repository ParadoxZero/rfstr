mod globals;

use atty::Stream;
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, BufRead},
};

use argust::ParserConfig;

fn main() {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    let arg_context = argust::parse_args(args.iter(), get_argust_config());

    let lines = get_lines(arg_context);

    println!("{:?}", lines);
}

fn get_lines(arg_context: argust::ArgContext) -> Vec<String> {
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

fn get_argust_config() -> Option<ParserConfig> {
    let mut parser_config = ParserConfig::new();
    parser_config.parse_tokens.option_key = " ".to_string();
    parser_config.add_parameter('p', "path");
    parser_config.add_parameter('s', "search");
    parser_config.add_parameter('m', "mode");

    return Some(parser_config);
}

fn get_lines_from_file(file_name: &str) -> Vec<String> {
    println!("path: {}", file_name);
    fs::read_to_string(file_name)
        .unwrap_or_else(move |err| panic!("Unable to read from: {}", err))
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
