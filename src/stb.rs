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

