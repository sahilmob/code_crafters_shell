pub fn echo(s: &str) {
    let mut args = s.split(" ");

    match args.next() {
        Some(_) => {
            println!("{}", args.collect::<Vec<&str>>().join(" "));
        }
        None => println!(""),
    }
}
