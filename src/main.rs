
use std::env;
use std::process;

use minigrep;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

      let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {  
        println!("Application error: {}", e);
        process::exit(1);

    }

}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename =  args[2].clone();
//         Ok(Config{query, filename})
//     }
// }










// use std::env;
// use std::fs;



// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = parse_config(&args);

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);

//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");

//     // --snip--
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// fn parse_config(args: &[String]) -> Config  {
//     let query = args[1].clone();
//     let filename =  args[2].clone();

//     Config{query, filename}
// }















//  gola
//$ cargo run searchstring example-filename.txt

//anti pattern to pac in into the tuple when we will neeed the single values 
// use std::env;
// use std::fs;


// fn main() {
//     // env::args() returns an iterator  we can collect
//     let args: Vec<String> = env::args().collect();

//     let (query, filename)  = parse_config(&args);

//  }

//  fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query,filename)

//  }






