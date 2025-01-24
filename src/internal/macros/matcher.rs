#[macro_export]
macro_rules! matcher {
    ($r: ident, $input: ident, $i:expr ) => {{
        let r = Regex::new($r).unwrap();
        let mut loc = r.capture_locations();
        r.captures_read_at(&mut loc, $input, $i);
        loc.get(0).unwrap_or((usize::MAX, usize::MAX)).0 == $i
    }};
}
