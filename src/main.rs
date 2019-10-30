use serenity::voice::AudioSource;
fn main() {
    println!("Hello, world!");

    let mut src = serenity::voice::ffmpeg("a-file").unwrap();
    dbg!(src.read_opus_frame());
}
