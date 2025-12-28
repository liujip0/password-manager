use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args);
    println!("{:?}", command);
}

#[derive(Debug)]
enum Command {
    None,
    Invalid(String),

    List,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Generate {
        key: String,
        special_chars: bool,
        length: usize,
    },
}

fn parse_args(args: &Vec<String>) -> Command {
    let command = match args.get(1) {
        Some(command) => command,
        None => return Command::None,
    };

    match command.as_str() {
        "list" => Command::List,
        "get" => {
            let key = match args.get(2) {
                Some(key) => key,
                None => return Command::Invalid("No key provided.".to_string()),
            };
            Command::Get { key: key.clone() }
        }
        "set" => {
            let key = match args.get(2) {
                Some(key) => key,
                None => return Command::Invalid("No key provided.".to_string()),
            };
            let value = match args.get(3) {
                Some(value) => value,
                None => return Command::Invalid("No value provided.".to_string()),
            };
            Command::Set {
                key: key.clone(),
                value: value.clone(),
            }
        }
        "generate" => {
            let key = match args.get(2) {
                Some(key) => key,
                None => return Command::Invalid("No key provided.".to_string()),
            };
            let special_chars = true;
            let length = 32;
            Command::Generate {
                key: key.clone(),
                special_chars,
                length,
            }
        }
        _ => Command::Invalid(format!("Unknown command: {}", command)),
    }
}
