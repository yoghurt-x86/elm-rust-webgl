use std::io::Write;

fn main() {
    let mut target = vec![];
    elm_rs::export!("Rust", &mut target, {
        // generates types and encoders for types implementing ElmEncoder
        encoders: [elm_rust::Msg, elm_rust::Global, elm_rust::Color, elm_rust::Skybox],
        // generates types and decoders for types implementing ElmDecoder
        decoders: [elm_rust::Msg, elm_rust::Global, elm_rust::Color, elm_rust::Event, elm_rust::Skybox],
    })
    .unwrap();
    let output = String::from_utf8(target).unwrap();
    let mut file = std::fs::File::create("../elm/bind/Rust.elm").unwrap();
    file.write_all(output.as_bytes()).expect("write failed");
}
