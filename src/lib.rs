use std::time::Instant;
use xcap::Monitor;
use chrono;

pub mod command {
    pub fn screen() {
        let path = String::from("target/");
        let start = super::Instant::now();
        let monitors = xcap::Monitor::all().unwrap();

        for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        let date = super::chrono::offset::Local::now();

        image
            .save(format!("{}/{}.png",path, date.format("%Y-%m-%d")))
            .unwrap();
    }
    }
}
