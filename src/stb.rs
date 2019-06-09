/*
Copyright (c) 2018-2019 Ethan Dagner.
All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use std::os::raw::{c_int, c_char, c_void};

use crate::Rgb;

// TODO: stb_image_write supports many other image formats than just PNG.
// think about allowing the user to specifiy a different output format.

const STB_RGB: c_int = 3;

pub fn stb_write_png(
    filename: &str,
    w: u32, h: u32,
    data: &Vec<Rgb>
) -> bool {
    use std::ffi::CString;

    unsafe {
        let c_filename = CString::new(filename).unwrap();
        stbi_write_png(
            c_filename.as_ptr(),
            w as c_int, h as c_int, STB_RGB,
            data.as_ptr() as *const c_void, STB_RGB * (w as c_int)) != 0
    }
}

#[link(name="stb_image_write")]
extern "C" {
    fn stbi_write_png(
        filename: *const c_char,
        w: c_int, h: c_int, comp: c_int,
        data: *const c_void, stride_in_bytes: c_int
    ) -> c_int;
}

