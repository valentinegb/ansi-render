use std::env::args;

use image::{GenericImageView, ImageReader, Pixel};

fn main() {
    let mut args = args();

    args.next();

    let img = ImageReader::open(args.next().expect("expected path to an image"))
        .unwrap()
        .decode()
        .unwrap();
    let img_height = img.height();

    for mut y in 0..img_height.div_ceil(2) {
        y *= 2;

        for x in 0..img.width() {
            const ESCAPE: char = '\x1B';
            const CONTROL_SEQUENCE_INTRODUCER: char = '[';
            const SELECT_GRAPHIC_RENDITION: char = 'm';
            const SET_FOREGROUND_COLOR: &str = "38";
            const SET_BACKGROUND_COLOR: &str = "48";
            const TWENTY_FOUR_BIT: &str = "02";

            let top_pixel = img.get_pixel(x, y).to_rgb();
            let top_channels = top_pixel.channels();
            let (top_r, top_g, top_b) = (top_channels[0], top_channels[1], top_channels[2]);

            print!(
                "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{SET_FOREGROUND_COLOR};{TWENTY_FOUR_BIT};{top_r};{top_g};{top_b}{SELECT_GRAPHIC_RENDITION}",
            );

            let bottom_y = y + 1;

            if bottom_y < img_height {
                let bottom_pixel = img.get_pixel(x, y + 1).to_rgb();
                let bottom_channels = bottom_pixel.channels();
                let (bottom_r, bottom_g, bottom_b) =
                    (bottom_channels[0], bottom_channels[1], bottom_channels[2]);

                print!(
                    "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{SET_BACKGROUND_COLOR};{TWENTY_FOUR_BIT};{bottom_r};{bottom_g};{bottom_b}{SELECT_GRAPHIC_RENDITION}",
                );
            }

            print!("▀");
        }

        println!();
    }
}
