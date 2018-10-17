extern crate image;
extern crate byteorder;
extern crate miniz_oxide;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Cursor, Read, Seek, SeekFrom};

use image::{ImageRgb8, ImageBuffer, Rgb};

use byteorder::{ReadBytesExt, LittleEndian};

use miniz_oxide::inflate::decompress_to_vec_zlib;

macro_rules! maybe_error {
    ($x:expr) => {
        match $x {
            Ok(ret) => ret,
            Err(err) => return Err(err.to_string())
        }
    };
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage : xyz2png [xyz file]");

        return Ok(());
    }

    let input_file = &args[1];
    let mut xyz_file: File = maybe_error!(File::open(input_file));
    let file_size = maybe_error!(xyz_file.seek(SeekFrom::End(0)));
    maybe_error!(xyz_file.seek(SeekFrom::Start(0)));

    let mut header: [u8; 4] = [0, 0, 0, 0];
    maybe_error!(xyz_file.read_exact(&mut header));

    if &header != b"XYZ1" {
        return Err("The input file is not an XYZ file.".to_owned());
    }

    let width = maybe_error!(xyz_file.read_u16::<LittleEndian>()) as u32;
    let height = maybe_error!(xyz_file.read_u16::<LittleEndian>()) as u32;

    let mut compressed_data = vec![0u8; (file_size - 8) as usize];
    maybe_error!(xyz_file.read_exact(&mut compressed_data));

    let uncompressed_data = match decompress_to_vec_zlib(&compressed_data) {
        Ok(data) => data,
        Err(_) => return Err("Failed to decompress image data.".to_owned())
    };

    let uncompressed_length = uncompressed_data.len();
    let mut data_cursor = Cursor::new(uncompressed_data);
    let mut palette: Vec<Rgb<u8>> = Vec::with_capacity(256);

    for _ in 0..256 {
        let r = maybe_error!(data_cursor.read_u8());
        let g = maybe_error!(data_cursor.read_u8());
        let b = maybe_error!(data_cursor.read_u8());

        palette.push(Rgb([r, g, b]));
    }

    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let pallete_index = data_cursor.read_u8().unwrap();

        palette[pallete_index as usize]
    });

    let mut output_file = String::from(Path::new(input_file).file_stem().unwrap().to_str().unwrap());
    output_file.push_str(".png");

    maybe_error!(ImageRgb8(buffer).save(Path::new(&output_file)));

    Ok(())
}
