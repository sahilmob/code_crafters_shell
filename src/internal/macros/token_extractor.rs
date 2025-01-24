#[macro_export]
macro_rules! token_extractor {
    ($r: ident, $input: ident, $i:ident ) => {{
        Regex::new($r)
            .unwrap()
            .captures(&$input[$i..])
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
    }};
}
