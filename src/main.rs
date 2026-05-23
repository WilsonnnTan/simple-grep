use std::{
    env, error::Error, fs, process
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_config = match Config::new(&args) {
        Ok(parsed_config) => parsed_config,
        Err(msg) => {
            eprintln!("Problem parsing arguments: {msg}");
            process::exit(1);
        },
    };

    if let Err(msg) = run(parsed_config) {
        eprintln!("Problem reading file: {}", msg);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("{}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3{
            return Err(String::from("Need atleast 2 args (Query and File Path)"));
        }

        let parsed_config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        };

        Ok(parsed_config)
    }
}

