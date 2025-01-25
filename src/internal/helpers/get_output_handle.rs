use std::fs::OpenOptions;
use std::io::Write;

fn parse_redirection(args: &Vec<String>) -> (Vec<String>, Option<String>, Option<String>) {
    let mut i = 0;
    let mut cmd_args: Vec<String> = Vec::new();
    let mut err_file = None;
    let mut output_file = None;
    while i < args.len() {
        if args[i] == ">" || args[i] == "1>" {
            if i + 2 < args.len() {
                output_file = Some(args[i + 2].clone());

                i += 3;
                continue;
            }
        } else if args[i] == "2>" {
            if i + 2 < args.len() {
                err_file = Some(args[i + 2].clone());

                i += 3;
                continue;
            }
        }

        cmd_args.push(args[i].clone());
        i += 1;
    }
    (cmd_args, output_file, err_file)
}

pub fn get_output_handle(cmds: &Vec<String>) -> (Vec<String>, Box<dyn Write>, Box<dyn Write>) {
    let (args, output_file, err_file) = parse_redirection(cmds);

    let handle: Box<dyn Write> = if let Some(output_file) = output_file {
        Box::new(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(output_file)
                .unwrap(),
        )
        // match std::fs::File::create(&output_file) {
        //     Ok(file) => Box::new(std::io::BufWriter::new(file)),
        //     Err(e) => {
        //         eprintln!("Failed to create output file {}: {}", output_file, e);
        //         Box::new(std::io::stdout())
        //     }
        // }
    } else {
        Box::new(std::io::stdout())
    };

    let err_handle: Box<dyn Write> = if let Some(err_file) = err_file {
        match std::fs::File::create(&err_file) {
            Ok(file) => Box::new(std::io::BufWriter::new(file)),
            Err(e) => {
                eprintln!("Failed to create error file {}: {}", err_file, e);
                Box::new(std::io::stdout())
            }
        }
    } else {
        Box::new(std::io::stdout())
    };

    (args, handle, err_handle)
}
