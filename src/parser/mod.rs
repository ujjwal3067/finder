#![allow(dead_code)]
use std::process;

#[derive(Debug)]
pub struct Args {
    pub flag: String,
    pub filename: String,
}

impl Args {
    pub fn new(filename: String, flag: String) -> Self {
        Args { flag, filename }
    }
}

fn exit_message(message: &str) -> ! {
    println!("[Error] : {}", message);
    help();
    process::exit(1);
}

fn help() {
    println!(
        "usage : 
    $./process  --flag filename  \n
    [flag] : 
        -d : Directory 
        -f : File
     "
    );
}

pub fn parse_arguments() -> Args {
    let args: Vec<_> = std::env::args().collect(); // first argument is process name
                                                   // error check
    match args.len() { 
        1..=2 =>  { 
            exit_message("Not enough arguments");
        }
        // correct arguments
        3 =>  { 
            // last two arguments are usefull for our use case 
            let filename = args[2].to_owned();
            let flag = args[1].to_owned();
            let arguments = Args::new(filename, flag);
            return arguments;
        }
        _ => { 
            exit_message("Wrong arguments");
        }
    }
}
