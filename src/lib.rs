use chrono;

pub mod command {
    pub fn screen(path: String) {
        let monitors = xcap::Monitor::all().unwrap();

        for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        let date = super::chrono::offset::Local::now();

        image
            .save(format!("{}{}.png",path, date.format("%Y-%m-%d")))
            .unwrap();
        }
    }

    pub fn help() {
        println!("Usage: hashcap [-s Capture the whole screen]\n\t       [-h List avaialable commands]");
    }
}
