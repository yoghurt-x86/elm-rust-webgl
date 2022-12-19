use std::io::Write;

fn main() {
    let mut target = vec![];
    elm_rs::export!("Rust", &mut target, {
        // generates types and encoders for types implementing ElmEncoder
        encoders: [elm_rust::Msg],
        // generates types and decoders for types implementing ElmDecoder
        decoders: [elm_rust::Msg],
    })
    .unwrap();
    let output = String::from_utf8(target).unwrap();
    let mut file = std::fs::File::create("../elm/bind/Rust.elm").unwrap();
    file.write_all(output.as_bytes()).expect("write failed");
}
