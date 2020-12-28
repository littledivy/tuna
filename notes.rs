const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

// https://newt.phys.unsw.edu.au/jw/notes.html
pub fn get_note(f: &f32) -> String {
    let m = 69.0 + 12.0 * (f / 440.0).log2();
    let e = m / 12.0 - 1.0;
    let e0 = e.round();
    return format!(
        "{} {}",
        NOTES[(m % 12.0) as usize].to_owned(),
        &(e - e0).to_string()
    );
}
