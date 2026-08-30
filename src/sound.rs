use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Use a system sound file or embed a small beep
    let file = File::open("/usr/share/sounds/freedesktop/stereo/bell.oga")?;
    let source = Decoder::new(BufReader::new(file))?;
    
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}
