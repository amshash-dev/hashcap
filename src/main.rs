use hashcap::{command, utils};
use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut path = PathBuf::from(format!(
        "{}/",
        env::current_dir()?
            .into_os_string()
            .into_string()
            .expect("Problem finding current working directory")
    ));
    let mut filename = String::new();

    if args.len() == 1 {
        return Ok(()); //TODO: add default functionality
    } else if args.len() == 3 {
        if args[2].starts_with("/") {
            path = PathBuf::from(args[2].clone());
        } else {
            path = PathBuf::from(format!(
                "{}{}",
                path.into_os_string().into_string().unwrap(),
                args[2]
            ));
        }
    } else if args.len() > 3 {
        return Err("Too many arguments")?;
    }
    if path.is_dir() {
        filename = "default".to_string();
    } else {
        filename = utils::gfnacp(&mut path);
    }

    let file_exists = utils::if_file_exists(&path, &filename);

    if file_exists {
        println!("File {}.png already exists", filename);
        return Ok(());
    }

    match args[1].as_str() {
        "--screen" => command::screen(path, filename),
        "--s" => command::screen(path, filename),
        "-screen" => command::screen(path, filename),
        "-s" => command::screen(path, filename),
        "--help" => command::help(),
        "--h" => command::help(),
        "-help" => command::help(),
        "-h" => command::help(),
        _ => todo!(), // make filename args[1] unless it begins with -
    }

    Ok(())
}
