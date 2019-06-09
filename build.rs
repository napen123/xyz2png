extern crate cc;

fn main() {
    cc::Build::new()
        .file("native/stb_image_write.c")
        .compile("stb_image_write");
}

