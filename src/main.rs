use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, thread, time::Duration};

fn main() {
    loop {
        // Send a notification
        Notification::new()
            .summary("Hydration Reminder")
            .body("Time to drink water!")
            .show()
            .unwrap();

        // Play a sound
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        let file = File::open("./water_drop.mp3").unwrap(); // Replace with your sound file
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.sleep_until_end();

        // Wait for 30 minutes
        thread::sleep(Duration::from_secs(1800)); // 1800 seconds = 30 minutes
    }
}
