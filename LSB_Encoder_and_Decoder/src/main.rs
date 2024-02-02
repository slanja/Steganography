use std::io;
use image::{GenericImage, GenericImageView, RgbaImage};

fn main() {
    encode();
    decode();
}

fn read_img() {
    // input image
    let img_input = image::open("images/testoutput.png").expect("File not found!");

    for (x, y, _pixel) in img_input.pixels() {
        println!("{:?}", _pixel)
    }
}

fn encode() {
    // loading input image
    let img_input = image::open("images/crab.png").expect("File not found!");

    // creating empty image
    let mut new_img = RgbaImage::new(img_input.width(), img_input.height());

    // message you want to hide
    let message = "Hello World!".to_string();

    // creating empty string for binary representation of our message
    let mut message_in_binary = "".to_string();

    // converting message to binary representation
    for character in message.clone().into_bytes() {
        message_in_binary += &format!("0{:b}", character);
    }
    println!("\"{}\" in binary is {}", message, message_in_binary);

    /*
    íčko se nezvyšuje, Koldy help

    - dodělat podmínky, aby workovalo dobře
    */

    let mut i = 0;

    // replacing each pixel in image with our message
    for (x, y, _pixel) in img_input.pixels() {

        let mut new_pixel_value = img_input.get_pixel(x, y);


        if new_pixel_value.0[0] == 0 { new_pixel_value.0[0] += 1}


        // red color
        if  new_pixel_value.0[0] % 2 == 0 && message_in_binary.chars().nth(i) == Some('1') && i < message_in_binary.len()-1 {
            new_pixel_value.0[0] += 1;
        }

        else if new_pixel_value.0[0] % 2 != 0 && message_in_binary.chars().nth(i) == Some('0') && i < message_in_binary.len()-1 {
            new_pixel_value.0[0] += 1;
        }
        i += 1;


        if new_pixel_value.0[1] == 0 { new_pixel_value.0[1] += 1}

        // green color
        if  new_pixel_value.0[1] % 2 == 0 && message_in_binary.chars().nth(i) == Some('1') && i < message_in_binary.len()-1 {
            new_pixel_value.0[1] += 1;
        }

        else if new_pixel_value.0[1] % 2 != 0 && message_in_binary.chars().nth(i) == Some('0') && i < message_in_binary.len()-1 {
            new_pixel_value.0[1] += 1;
        }
        i += 1;



        if new_pixel_value.0[2] == 0 { new_pixel_value.0[2] += 1}

        // blue color
        if  new_pixel_value.0[2] % 2 == 0 && message_in_binary.chars().nth(i) == Some('1') && i < message_in_binary.len()-1 {
            new_pixel_value.0[2] += 1;
        }

        else if new_pixel_value.0[2] % 2 != 0 && message_in_binary.chars().nth(i) == Some('0') && i < message_in_binary.len()-1 {
            new_pixel_value.0[2] += 1;
        }
        i += 1;


        if new_pixel_value.0[3] == 0 { new_pixel_value.0[3] += 1}

        // alfa channel
        if  new_pixel_value.0[3] % 2 == 0 && message_in_binary.chars().nth(i) == Some('1') && i < message_in_binary.len()-1 {
            new_pixel_value.0[3] += 1;
        }

        else if new_pixel_value.0[3] % 2 != 0 && message_in_binary.chars().nth(i) == Some('0') && i < message_in_binary.len()-1 {
            new_pixel_value.0[3] += 1;
        }
        i += 1;


        // update the new_img pixel with the modified new_pixel_value
        new_img.put_pixel(x, y, new_pixel_value);

        if i < message_in_binary.len()-1 {
            print!("{:?} ", img_input.get_pixel(x, y));
            print!("- {:?}\n", new_pixel_value);
        }
    }

    new_img.save("images/output.png").expect("Failed to save new image.");

}

fn decode() {
    // input image
    let img_input = image::open("images/output.png").expect("File not found!");

    // empty message in binary
    let mut message_in_binary = "".to_string();

    // let mut count = 0;

    // extracting message from image
    for (x, y, _pixel) in img_input.pixels() {
        let pixel_value = img_input.get_pixel(x, y);

        // red color
        if pixel_value.0[0] % 2 == 0 { message_in_binary += "0"; } else { message_in_binary += "1"; }

        // green color
        if pixel_value.0[1] % 2 == 0 { message_in_binary += "0"; } else { message_in_binary += "1"; }

        // blue color
        if pixel_value.0[2] % 2 == 0 { message_in_binary += "0"; } else { message_in_binary += "1"; }

        // alfa channel
        if pixel_value.0[3] % 2 == 0 { message_in_binary += "0"; } else { message_in_binary += "1"; }

        /*count += 1;

        if count == 2 {
            message_in_binary += " ";
            count = 0;
        }*/
    }

    println!("{}", message_in_binary);
}