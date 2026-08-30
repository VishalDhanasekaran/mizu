#[cfg(feature = "sound")]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let file = File::open("/usr/share/sounds/freedesktop/stereo/bell.oga")?;
    let source = Decoder::new(BufReader::new(file))?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

#[cfg(not(feature = "sound"))]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
