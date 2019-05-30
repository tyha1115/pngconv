extern crate png;

use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!();

    if args.len() < 3 {
        println!("PNG converter v1.0");
        println!("Usage: {} [input] [output]", args[0]);
        std::process::exit(0);
    }

    let decoder = png::Decoder::new(std::fs::File::open(&args[1])?);
    let (info, mut reader) = decoder.read_info()?;

    println!("[PNG Info]");
    println!("- width: {}", info.width);
    println!("- height: {}", info.height);
    println!("- bit_depth: {:?}", info.bit_depth);
    println!("- color_type: {:?}", info.color_type);

    if info.bit_depth != png::BitDepth::Eight || info.color_type != png::ColorType::RGBA {
        println!("Error: bit_depth or color_type is not supported!");
        std::process::exit(0);
    }

    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];

    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf)?;

    let mut output = std::fs::File::create(&args[2])?;

    write!(&mut output, "#include <stdint.h>\n\n")?;
    write!(&mut output, "uint8_t buf[{}] = {{\n", info.buffer_size() + 56)?; // 56 means the header size

    // Write header
    write!(&mut output, "\t0x00, 0x00, 0x00, 0x00, // uint32_t id\n")?;
    write!(&mut output, "\t0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, // int32_t width\n",
        (info.width & 0x000000ff),
        (info.width & 0x0000ff00) >> 8,
        (info.width & 0x00ff0000) >> 16,
        (info.width & 0xff000000) >> 24)?;
    write!(&mut output, "\t0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, // int32_t height\n",
        (info.height & 0x000000ff),
        (info.height & 0x0000ff00) >> 8,
        (info.height & 0x00ff0000) >> 16,
        (info.height & 0xff000000) >> 24)?;
    write!(&mut output, "\t0x08, 0x00, 0x00, 0x00, // ui_pixel_format_t pf\n")?;
    write!(&mut output, "\t0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, // uint32_t header_size\n",
        (56u32 & 0x000000ff),
        (56u32 & 0x0000ff00) >> 8,
        (56u32 & 0x00ff0000) >> 16,
        (56u32 & 0xff000000) >> 24)?;
    write!(&mut output, "\t0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, // uint32_t data_size\n",
        (info.buffer_size() & 0x000000ff),
        (info.buffer_size() & 0x0000ff00) >> 8,
        (info.buffer_size() & 0x00ff0000) >> 16,
        (info.buffer_size() & 0xff000000) >> 24)?;

    for i in 0..8 {
        write!(&mut output, "\t0x00, 0x00, 0x00, 0x00, // int32_t reserved[{}]\n", i)?;
    }

    let mut count = 0;
    for i in (0..info.buffer_size()).step_by(4) {
        let r: u8 = buf[i];
        let g: u8 = buf[i + 1];
        let b: u8 = buf[i + 2];
        let a: u8 = buf[i + 3];
        let hibyte: u8 = (r & 0b11111000) | ((g & 0b11100000) >> 5);
        let lobyte: u8 = ((g & 0b00011100) << 3) | (b & 0b00011111);

        if count % 16 == 0 {
            write!(&mut output, "\t")?;
        }

        write!(&mut output, "0x{:02x}, 0x{:02x}, 0x{:02x}", hibyte, lobyte, a)?;

        if count != ((info.buffer_size() / 4) - 1) {
            write!(&mut output, ", ")?;
        }

        if (count + 1) % 16 == 0 {
            write!(&mut output, "\n")?;
        }

        count = count + 1;
    }

    write!(&mut output, "}};\n\n")?;

    println!("\n{} created successfully!", &args[2]);

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