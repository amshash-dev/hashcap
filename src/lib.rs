use std::path::PathBuf;

pub mod utils {
    pub fn gfnacp(path: &mut super::PathBuf) -> String {
        let mut ret_string = String::new();
        let path_str = <super::PathBuf as Clone>::clone(&path)
            .into_os_string()
            .into_string()
            .unwrap();

        for (_i, c) in path_str.chars().rev().enumerate() {
            if c == '/' {
                break;
            }

            ret_string.push_str(&c.to_string());
        }

        path.pop();

        return ret_string.chars().rev().collect();
    }

    pub fn if_file_exists(path: &super::PathBuf, filename: &String) -> bool {
        let mut full_path: super::PathBuf = path.clone();
        let mut full_filename = filename.clone();
        full_filename.push_str(".png");
        full_path.push(full_filename);

        return full_path.exists();
    }
}
pub mod command {
    pub fn screen(path: super::PathBuf, filename: String) {
        let monitors = xcap::Monitor::all().unwrap();

        for monitor in monitors {
            let image = monitor.capture_image().unwrap();

            image
                .save(format!("{}/{}.png", path.display(), filename))
                .unwrap();
            // TODO: match these errors for nicer message on no directory
        }
    }

    pub fn help() {
        println!(
            "Usage: hashcap [-s Capture the whole screen]\n\t       [-h List avaialable commands]"
        );
    }
}
