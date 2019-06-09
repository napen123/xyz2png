/*
Copyright (c) 2018-2019, Ethan Dagner.
All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

extern crate byteorder;
extern crate miniz_oxide;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LittleEndian};

use miniz_oxide::inflate::decompress_to_vec_zlib;

mod stb;
use stb::stb_write_png;

macro_rules! maybe_error {
    ($x:expr) => {
        match $x {
            Ok(ret) => ret,
            Err(err) => return Err(err.to_string())
        }
    };
}

#[derive(Clone, Copy, Default)]
pub struct Rgb(u8, u8, u8);

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();

    if arg_len != 2 && arg_len != 3 {
        println!("Usage:\n   xyz2png [xyz file]\n   xyz2png [xyz file] [output file]");

        return Ok(());
    }

    let input_file = &args[1];
    let mut xyz_file: File = maybe_error!(File::open(input_file));

    let output_file = if arg_len == 3 {
        args[2].to_owned()
    } else {
        let output = Path::new(input_file);
        let output_ext = output.with_extension("png");

        output_ext
            .to_str()
            .unwrap()
            .to_owned()
    };

    let file_size = maybe_error!(xyz_file.seek(SeekFrom::End(0)));
    maybe_error!(xyz_file.seek(SeekFrom::Start(0)));

    let mut header: [u8; 4] = [0, 0, 0, 0];
    maybe_error!(xyz_file.read_exact(&mut header));

    if &header != b"XYZ1" {
        return Err("The provided file is not a valid XYZ file.".to_owned());
    }

    let width = maybe_error!(xyz_file.read_u16::<LittleEndian>()) as u32;
    let height = maybe_error!(xyz_file.read_u16::<LittleEndian>()) as u32;

    let mut compressed_data = vec![0u8; (file_size - 8) as usize];
    maybe_error!(xyz_file.read_exact(&mut compressed_data));

    let uncompressed_data = match decompress_to_vec_zlib(&compressed_data) {
        Ok(data) => data,
        Err(_) => return Err("Failed to decompress image data.".to_owned())
    };

    let mut data_cursor = Cursor::new(uncompressed_data);
    let mut palette: Vec<Rgb> = Vec::with_capacity(256);

    for _ in 0..256 {
        let r = maybe_error!(data_cursor.read_u8());
        let g = maybe_error!(data_cursor.read_u8());
        let b = maybe_error!(data_cursor.read_u8());

        palette.push(Rgb(r, g, b));
    }

    let image_size = (width * height) as usize;
    let mut image_data: Vec<Rgb> = Vec::with_capacity(image_size);

    for _ in 0..image_size {
        // TODO: There's definitely a better way of doing this; this can't be fast.
        let pallete_index = maybe_error!(data_cursor.read_u8());
        image_data.push(palette[pallete_index as usize]);
    }

    if stb_write_png(&output_file, width, height, &image_data) {
        Ok(())
    } else {
        Err("Failed to write PNG file.".to_owned())
    }
}

