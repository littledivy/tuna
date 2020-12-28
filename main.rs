mod notes;
mod stream;

fn main() {
    println!("Starting stream");
    stream::InputStream::new().start();
}
