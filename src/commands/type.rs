use crate::{echo, exit};

pub static TYPE: &str = "type";

#[macro_export]
macro_rules! typ {
    ($($key: path),*) => {
        {
            use ::std::collections::HashMap;
            let mut hm: HashMap<&str, String> = HashMap::new();

            $ (
                let mut s = String::from($key.to_string());
                s.push_str(" is a shell builtin");
                hm.insert($key, s);
            )*

            hm
       }
    };
}

pub fn typ(args: Vec<&str>) {
    if args.is_empty() {
        println!("not found")
    }

    let hm = typ!(exit::TYPE, echo::TYPE, self::TYPE);

    match hm.get(args.last().unwrap()) {
        Some(v) => println!("{}", v),
        None => println!("{}: not found", args.last().unwrap()),
    };
}
