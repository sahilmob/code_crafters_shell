use std::io::Write;

fn parse_redirection(args: &Vec<String>) -> (Vec<String>, Option<String>) {
    let mut i = 0;
    let mut cmd_args: Vec<String> = Vec::new();
    let mut output_file = None;
    while i < args.len() {
        if args[i] == ">" || args[i] == "1>" {
            if i + 2 < args.len() {
                output_file = Some(args[i + 2].clone());

                i += 3;
                continue;
            }
        }
        cmd_args.push(args[i].clone());
        i += 1;
    }
    (cmd_args, output_file)
}

pub fn get_output_handle(cmds: &Vec<String>) -> (Vec<String>, Box<dyn Write>) {
    let (args, output_file) = parse_redirection(cmds);

    let handle: Box<dyn Write> = if let Some(output_file) = output_file {
        match std::fs::File::create(&output_file) {
            Ok(file) => Box::new(std::io::BufWriter::new(file)),
            Err(e) => {
                eprintln!("Failed to create output file {}: {}", output_file, e);
                Box::new(std::io::stdout())
            }
        }
    } else {
        Box::new(std::io::stdout())
    };

    (args, handle)
}
