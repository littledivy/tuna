use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use soundio::Context;
fn main() {
    let mut ctx = Context::new();
    ctx.set_app_name("tuner-rs");
    ctx.connect().unwrap();
    ctx.flush_events();
    let dev = ctx.default_input_device().expect("No input device");

    let mut input_stream = dev
        .open_instream(
            44100,
            soundio::Format::U8,
            soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo),
            0.0,
            read_callback,
            None::<fn()>,
            None::<fn(soundio::Error)>,
        )
        .unwrap();

    input_stream.start().unwrap();
    println!("Starting stream");

    // Wait for the user to press a key.
    println!("Press enter to stop recording");
    let stdin = std::io::stdin();
    let input = &mut String::new();
    let _ = stdin.read_line(input);
}

const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

fn read_callback(stream: &mut soundio::InStreamReader) {
    let frame_count_max = stream.frame_count_max();
    if let Err(e) = stream.begin_read(frame_count_max) {
        println!("Error reading from stream: {}", e);
        return;
    }

    let mut samples: Vec<f32> = vec![];
    for f in 0..stream.frame_count() {
        for c in 0..stream.channel_count() {
            samples.push(stream.sample::<f32>(c, f));
        }
    }

    let mut detector = McLeodDetector::new(samples.len(), samples.len() / 2);
    if let Some(pitch) = detector.get_pitch(&samples, 44100, 5.0, 0.7) {
        let note = get_note(&pitch.frequency);
        print!("\x1B[2J\x1B[1;1H");
        println!("Note: {}", note);
        println!("Frequency: {}, Clarity: {}", pitch.frequency, pitch.clarity);
    }
}

// https://newt.phys.unsw.edu.au/jw/notes.html
fn get_note(f: &f32) -> String {
    let m = 69.0 + 12.0 * (f / 440.0).log2();
    let e = m / 12.0 - 1.0;
    let e0 = e.round();
    return format!(
        "{} {}",
        NOTES[(m % 12.0) as usize].to_owned(),
        &(e - e0).to_string()
    );
}
