use std::{
    env, error::Error, fs, process
};
use simplegrep;

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
    let query = config.query;
    
    let searched_result: Vec<&str>;

    if config.is_case_sensitive {
        searched_result = simplegrep::search_case_sensitive(&query, &contents);
    } else {
        searched_result = simplegrep::search_case_insensitive(&query, &contents);
    }


    for res in searched_result {
        println!("{res}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    is_case_sensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(String::from("Need atleast 2 args (Query and File_Path)"));
        }

        let is_case_sensitive = env::var("IS_CASE_SENSITIVE").is_ok();

        let parsed_config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            is_case_sensitive,
        };

        Ok(parsed_config)
    }
}

