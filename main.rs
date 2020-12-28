mod notes;
mod stream;
mod ui;

fn main() {
    println!("Starting stream");
    stream::InputStream::new().start();
}
