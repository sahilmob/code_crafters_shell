use regex::Regex;

use crate::{matcher, token_extractor};

static SPACE: &str = r#" +"#;
static BACK_SLASH: &str = r"\\";
static SINGLE_QUOTE: &str = "'";
static DOUBLE_QUOTE: &str = "\"";
static CMD_SEGMENT: &str = r#"[^ '"\\]+"#;
static ESCAPE_SYMBOLS: &str = r#"[\\$"]+"#;
static BETWEEN_SINGLE_QUOTES: &str = r#"[^']*"#;
static BETWEEN_DOUBLE_QUOTES: &str = r#"(?:[^"\\]|\\.)*"#;

fn parse_between_double_quotes(input: &str) -> String {
    let mut i = 0;
    let mut matched_escape = false;
    let mut result = String::new();

    while i < input.len() {
        if input[i..i + 1] == *"\\" && !matched_escape && matcher!(ESCAPE_SYMBOLS, input, i + 1) {
            matched_escape = true;
            i += 1;
            continue;
        }

        result.push_str(&input[i..i + 1]);

        if matched_escape {
            matched_escape = false;
        }

        i += 1;
    }

    result
}

fn match_single_quote(input: &str, i: usize) -> bool {
    matcher!(SINGLE_QUOTE, input, i)
    // let r = Regex::new(SINGLE_QUOTE).unwrap();
    // let mut loc = r.capture_locations();
    // r.captures_read_at(&mut loc, input, i);
    // loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_double_quote(input: &str, i: usize) -> bool {
    matcher!(DOUBLE_QUOTE, input, i)
    // let r = Regex::new(DOUBLE_QUOTE).unwrap();
    // let mut loc = r.capture_locations();
    // r.captures_read_at(&mut loc, input, i);
    // loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_back_slash(input: &str, i: usize) -> bool {
    matcher!(BACK_SLASH, input, i)
    // let r = Regex::new(BACK_SLASH).unwrap();
    // let mut loc = r.capture_locations();
    // r.captures_read_at(&mut loc, input, i);
    // loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_space(input: &str, i: usize) -> bool {
    matcher!(SPACE, input, i)
    // let r = Regex::new(SPACE).unwrap();
    // let mut loc = r.capture_locations();
    // r.captures_read_at(&mut loc, input, i);
    // loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_cmd_segment(input: &str, i: usize) -> bool {
    matcher!(CMD_SEGMENT, input, i)
    // let r = Regex::new(CMD_SEGMENT).unwrap();
    // let mut loc = r.capture_locations();
    // r.captures_read_at(&mut loc, input, i);
    // loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

pub fn parse(input: String) -> Vec<String> {
    let mut i = 0;
    let input = input.trim();
    let mut result = Vec::new();

    while i < input.len() {
        if match_cmd_segment(input, i) {
            let token = token_extractor!(CMD_SEGMENT, input, i);
            // let token = Regex::new(CMD_SEGMENT)
            //     .unwrap()
            //     .captures(&input[i..])
            //     .unwrap()
            //     .get(0)
            //     .unwrap()
            //     .as_str()
            //     .to_string();

            result.push(token.clone());

            i += eat(&token);
        } else if match_double_quote(input, i) {
            i += eat(DOUBLE_QUOTE);
            let token = token_extractor!(BETWEEN_DOUBLE_QUOTES, input, i);
            let parsed_token = parse_between_double_quotes(token.as_str());

            // let token = Regex::new(BETWEEN_DOUBLE_QUOTES)
            //     .unwrap()
            //     .captures(&input[i..])
            //     .unwrap()
            //     .get(0)
            //     .unwrap()
            //     .as_str()
            //     .to_string();

            if !parsed_token.is_empty() {
                result.push(parsed_token);
                i += eat(&token);
            }
            i += eat(DOUBLE_QUOTE);
        } else if match_single_quote(input, i) {
            i += eat(SINGLE_QUOTE);

            let token: String = token_extractor!(BETWEEN_SINGLE_QUOTES, input, i);

            // let token = Regex::new(BETWEEN_SINGLE_QUOTES)
            //     .unwrap()
            //     .captures(&input[i..])
            //     .unwrap()
            //     .get(0)
            //     .unwrap()
            //     .as_str()
            //     .to_string();

            if !token.is_empty() {
                result.push(token.clone());
                i += eat(&token);
            }

            i += eat(SINGLE_QUOTE);
        } else if match_back_slash(input, i) {
            i += eat("\\");
            result.push(input[i..i + 1].to_string());
            i += 1;
        } else if match_space(input, i) {
            let token: String = token_extractor!(SPACE, input, i);

            // let token = Regex::new(SPACE)
            //     .unwrap()
            //     .captures(&input[i..])
            //     .unwrap()
            //     .get(0)
            //     .unwrap()
            //     .as_str()
            //     .to_string();

            result.push(" ".to_string());
            i += eat(&token);
        } else {
            panic!("Unexpected token at {}", i);
        }
    }

    result
}

fn eat(token: &str) -> usize {
    token.len()
}
