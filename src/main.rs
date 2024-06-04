use std::{env, error::Error};
use hashcap::command;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut path: String = format!("{}/", env::current_dir()?.into_os_string().into_string().expect("Problem finding current working directory"));

    if args.len() == 1 {
        return Ok(()); //todo add default functionality
    } else if args.len() == 3 {
        if args[2].contains("/") {
            path = args[2].clone();
        } else {
            path.push_str(&args[2].clone());
        }
    } else if args.len() > 3 {
        println!("Too many arguments");
        return Ok(());
    }

    

    match args[1].as_str() {
        "--screen" => command::screen(path),
        "--s" => command::screen(path),
        "-screen" => command::screen(path),
        "-s" => command::screen(path),
        "--help" => command::help(),
        "--h" => command::help(),
        "-help" => command::help(),
        "-h" => command::help(),
        _ => todo!(), // make filename args[1] unless it begins with -
    }

    Ok(())
}
