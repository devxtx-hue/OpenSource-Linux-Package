use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    
    if args.len() == 1 {
        return;
    }
    
    let mut interpret_escapes = false;
    let mut no_newline = false;
    let mut text_args = Vec::new();
    
    // parse flags
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-e" => interpret_escapes = true,
            "-E" => interpret_escapes = false,
            "-n" => no_newline = true,
            _ if arg.starts_with('-') => {
                eprintln!("N/A option: {}", arg);
                return;
            }
            _ => text_args.push(arg.clone()),
        }
    }
    
    let mut output = text_args.join(" ");
    
    if interpret_escapes {
        output = output
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");
    }
    
    if no_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
}