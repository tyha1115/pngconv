extern crate png;

use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!();

    if args.len() < 2 {
        println!("PNG converter v1.0");
        println!("Usage: {} [pngfile]", args[0]);
        std::process::exit(0);
    }

    let decoder = png::Decoder::new(std::fs::File::open("setupwizard_ic_gear.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    println!("[PNG Info]");

    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];

    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();

    let mut output = std::fs::File::create("foo.txt").unwrap();

    let data = b"some bytes";

    for i in (0..info.buffer_size()).step_by(4) {
        let r: u16 = buf[i] as u16;
        let g: u16 = buf[i + 1] as u16;
        let b: u16 = buf[i + 2] as u16;
        let a: u16 = buf[i + 3] as u16;

        let rgb565 = ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3);
    }

    output.write_all(b"Hello File!").unwrap();

    Ok(())
}

/*
use std::fs::File;
use std::io::Write;

fn main() {
    let data = "Some data!";
    let mut f = File::create("/tmp/foo").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
*/