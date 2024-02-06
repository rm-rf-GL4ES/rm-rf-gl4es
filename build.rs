use bindgen::builder;

fn main() {
    let gles_bindings = builder()
        .header("headers/gl.h")
        .generate().unwrap();
    gles_bindings.write_to_file("src/bindings/gles_bindings.rs").unwrap();

    let egl_bindings = builder()
        .header("headers/egl.h")
        .generate().unwrap();
    egl_bindings.write_to_file("src/bindings/egl_bindings.rs").unwrap();
}