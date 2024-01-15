use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

fn main() {
    encode()
}

fn encode() {
    // input image
    let img_input = image::open("images/crab.png").expect("File not found!");

    // creating empty image
    let mut new_img = RgbaImage::new(img_input.width(), img_input.height());

    // message you want to hide
    let message = "Hello Rust!".to_string();

    // creating empty string for binary representation of our message
    let mut message_in_binary = "".to_string();

    // converting message to binary representation
    for character in message.clone().into_bytes() {
        message_in_binary += &format!("0{:b}", character);
    }
    println!("\"{}\" in binary is {}", message, message_in_binary);

    // replacing each pixel in image with our message
    let mut i = 0;

    for (x, y, pixel) in img_input.pixels() {
        let mut new_pixel_value = img_input.get_pixel(x, y);;

        // red color
        if  new_pixel_value.0[0] % 2 == 0 {
            if new_pixel_value.0[0] < 255 {
                new_pixel_value.0[0] += 1;
            } else { new_pixel_value.0[0] -= 1;}
        }

        // green color
        if  new_pixel_value.0[1] % 2 == 0 {
            if new_pixel_value.0[1] < 255 {
                new_pixel_value.0[1] += 1;
            } else { new_pixel_value.0[1] -= 1;}
        }

        // blue color
        if  new_pixel_value.0[2] % 2 == 0 {
            if new_pixel_value.0[2] < 255 {
                new_pixel_value.0[2] += 1;
            } else { new_pixel_value.0[2] -= 1;}
        }

        // alfa channel
        if  new_pixel_value.0[3] % 2 == 0 {
            if new_pixel_value.0[3] < 255 {
                new_pixel_value.0[3] += 1;
            } else { new_pixel_value.0[3] -= 1;}
        }

        new_img.put_pixel(x, y, new_pixel_value);
        println!("{:?}", new_pixel_value);
        i += 1;
    }

    new_img.save("images/output.png").expect("Failed to save new image.");

}