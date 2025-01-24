use regex::Regex;

static SPACE: &str = r#" +"#;
static SINGLE_QUOTE: &str = "'";
static DOUBLE_QUOTE: &str = "\"";
static CMD_SEGMENT: &str = r#"[^ '"]+"#;
static BETWEEN_SINGLE_QUOTES: &str = r#"[^']*"#;
static BETWEEN_DOUBLE_QUOTES: &str = r#"[^"]*"#;

fn match_single_quote(input: &str, i: usize) -> bool {
    let r = Regex::new(SINGLE_QUOTE).unwrap();
    let mut loc = r.capture_locations();
    r.captures_read_at(&mut loc, input, i);
    loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_double_quote(input: &str, i: usize) -> bool {
    let r = Regex::new(DOUBLE_QUOTE).unwrap();
    let mut loc = r.capture_locations();
    r.captures_read_at(&mut loc, input, i);
    loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_space(input: &str, i: usize) -> bool {
    let r = Regex::new(SPACE).unwrap();
    let mut loc = r.capture_locations();
    r.captures_read_at(&mut loc, input, i);
    loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_cmd_segment(input: &str, i: usize) -> bool {
    let r = Regex::new(CMD_SEGMENT).unwrap();
    let mut loc = r.capture_locations();
    r.captures_read_at(&mut loc, input, i);
    loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

pub fn parse(input: String) -> Vec<String> {
    let mut i = 0;
    let input = input.trim();
    let mut result = Vec::new();

    while i < input.len() {
        if match_cmd_segment(input, i) {
            let token = Regex::new(CMD_SEGMENT)
                .unwrap()
                .captures(&input[i..])
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .to_string();

            result.push(token.clone());

            i += eat(&token);
        } else if match_double_quote(input, i) {
            i += eat(DOUBLE_QUOTE);

            let token = Regex::new(BETWEEN_DOUBLE_QUOTES)
                .unwrap()
                .captures(&input[i..])
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .to_string();

            if !token.is_empty() {
                result.push(token.clone());
                i += eat(&token);
            }

            i += eat(DOUBLE_QUOTE);
        } else if match_single_quote(input, i) {
            i += eat(SINGLE_QUOTE);

            let token = Regex::new(BETWEEN_SINGLE_QUOTES)
                .unwrap()
                .captures(&input[i..])
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .to_string();

            if !token.is_empty() {
                result.push(token.clone());
                i += eat(&token);
            }

            i += eat(SINGLE_QUOTE);
        } else if match_space(input, i) {
            let token = Regex::new(SPACE)
                .unwrap()
                .captures(&input[i..])
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .to_string();

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
