extern crate image;
extern crate qrcode;

mod bee;

fn main() {
    bee::qr::generate("hi");
}
