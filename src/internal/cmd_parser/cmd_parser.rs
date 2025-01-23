use regex::Regex;

static SPACE: &str = " ";
static SINGLE_QUOTE: &str = "'";
static CMD_SEGMENT: &str = r#"[^ ']+"#;
static BETWEEN_SINGLE_QUOTE: &str = r#"[^']*"#;

fn match_single_quote(input: &str, i: usize) -> bool {
    let r = Regex::new(SINGLE_QUOTE).unwrap();
    let mut loc = r.capture_locations();
    r.captures_read_at(&mut loc, input, i);
    loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == i
}

fn match_space(input: &str, i: usize) -> bool {
    input[i..].starts_with(SPACE)
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
        } else if match_single_quote(input, i) {
            i += eat(SINGLE_QUOTE);

            let token = Regex::new(BETWEEN_SINGLE_QUOTE)
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
            result.push(SPACE.to_string());
            i += eat(SPACE)
        } else {
            panic!("Unexpected token at {}", i);
        }
    }

    result
}

fn eat(token: &str) -> usize {
    token.len()
}
