use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

fn main() {
    test()
}

fn test() {

    let img_input = image::open("images/crab.png").expect("File not found!");

    let mut new_img = RgbaImage::new(img_input.width(), img_input.height());

    // message you want to hide
    let message = "Hello Rust!".to_string();

    // creating empty string for binary representation of our message
    let mut message_in_binary = "".to_string();

    // converting message to binary representation
    for character in message.clone().into_bytes() {
        message_in_binary += &format!("0{:b} ", character);
    }
    println!("\"{}\" in binary is {}", message, message_in_binary);

    // replacing each pixel in image with our message
    for (x, y, pixel) in img_input.pixels() {
        let new_pixel_value = Rgba([255, 255, 0, 255]);
        new_img.put_pixel(x, y, new_pixel_value);
    }

    new_img.save("images/output.png").expect("Failed to save new image.");

}