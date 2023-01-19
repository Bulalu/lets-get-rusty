use  std::env;
use  std::fs;
use  std::process;



fn main() {
    let args:Vec<String> = env::args().collect();

    // let config = parse_config(& args);
    let config = Config::build(&args).unwrap_or_else(
        |err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );

    println!("Searching for {}", config.query);
    println!("In File {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}")
    
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query: query, file_path: file_path })
    }
}


