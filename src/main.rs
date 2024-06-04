use std::env;

use hashcap::command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return; //todo add default functionality
    }

    match args[1].as_str() {
        "--screen" => command::screen(),
        "--s" => command::screen(),
        "-screen" => command::screen(),
        "-s" => command::screen(),
        "--help" => command::help(),
        "--h" => command::help(),
        "-help" => command::help(),
        "-h" => command::help(),
        _ => todo!(),
    }
}
