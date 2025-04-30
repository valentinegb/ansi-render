#![feature(test)]

use std::{env::args, fmt::Write, path::Path};

use image::{GenericImageView, ImageReader, Pixel};

fn render_image(path: impl AsRef<Path>) {
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let img_height = img.height();
    let mut buf = String::new();

    for mut y in 0..img_height.div_ceil(2) {
        y *= 2;

        for x in 0..img.width() {
            const ESCAPE: char = '\x1B';
            const CONTROL_SEQUENCE_INTRODUCER: char = '[';
            const SELECT_GRAPHIC_RENDITION: char = 'm';
            const RESET: &str = "0";
            const SET_FOREGROUND_COLOR: &str = "38";
            const SET_BACKGROUND_COLOR: &str = "48";
            const TWENTY_FOUR_BIT: &str = "2";

            let top_pixel = img.get_pixel(x, y);
            let top_channels = top_pixel.channels();
            let (top_r, top_g, top_b, top_a) = (
                top_channels[0],
                top_channels[1],
                top_channels[2],
                top_channels[3],
            );
            let top_rendered = top_a != 0;
            let bottom_y = y + 1;
            let bottom_channels = (bottom_y < img_height).then(|| {
                let bottom_pixel = img.get_pixel(x, y + 1);
                let bottom_channels = bottom_pixel.channels();

                (
                    bottom_channels[0],
                    bottom_channels[1],
                    bottom_channels[2],
                    bottom_channels[3],
                )
            });
            let bottom_rendered = bottom_channels
                .is_some_and(|(_bottom_r, _bottom_g, _bottom_b, bottom_a)| bottom_a != 0);

            if top_rendered {
                write!(
                    &mut buf,
                    "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{SET_FOREGROUND_COLOR};{TWENTY_FOUR_BIT};{top_r};{top_g};{top_b}{SELECT_GRAPHIC_RENDITION}",
                ).unwrap();

                if bottom_rendered {
                    let (bottom_r, bottom_g, bottom_b, _bottom_a) = bottom_channels.unwrap();

                    write!(
                        &mut buf,
                        "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{SET_BACKGROUND_COLOR};{TWENTY_FOUR_BIT};{bottom_r};{bottom_g};{bottom_b}{SELECT_GRAPHIC_RENDITION}",
                    ).unwrap();
                }

                write!(&mut buf, "▀").unwrap();
            } else if bottom_rendered {
                let (bottom_r, bottom_g, bottom_b, _bottom_a) = bottom_channels.unwrap();

                write!(
                    &mut buf,
                    "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{SET_FOREGROUND_COLOR};{TWENTY_FOUR_BIT};{bottom_r};{bottom_g};{bottom_b}{SELECT_GRAPHIC_RENDITION}▄",
                ).unwrap();
            } else {
                write!(&mut buf, " ").unwrap();
            }

            write!(
                &mut buf,
                "{ESCAPE}{CONTROL_SEQUENCE_INTRODUCER}{RESET}{SELECT_GRAPHIC_RENDITION}",
            )
            .unwrap();
        }

        writeln!(&mut buf).unwrap();
    }

    print!("{buf}");
}

fn main() {
    let mut args = args();

    args.next();

    render_image(args.next().expect("expected path to an image"));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn small_image(b: &mut Bencher) {
        b.iter(|| render_image("assets/bench_small.webp"));
    }

    #[bench]
    fn large_image(b: &mut Bencher) {
        b.iter(|| render_image("assets/bench_large.jpeg"));
    }
}
