use image::Luma;
use qrcode::QrCode;

pub fn generate(data: &str) {
    // let code = QrCode::new(b"hello").unwrap();
    //
    // let rgb_r = code.render::<Luma<u8>>();
    // let image = rgb_r
    //     // .dark_color(Rgb([0, 0, 128]))
    //     // .light_color(Rgb([224, 224, 224])) // adjust colors
    //     // .quiet_zone(false) // disable quiet zone (white border)
    //     // .min_dimensions(300, 300) // sets minimum image size
    //     .build();
    // image.save("/tmp/qrcode.png").unwrap();
    //
    // // You can also render it into a string.
    // let char_r = code.render();
    // let string = char_r.light_color(' ').dark_color('#').build();
    // println!("{}", string);
}

#[test]
fn it_generate() {
    generate("hello")
}
