use image::open;
use rayon::prelude::*;
use std::path::Path;

fn main() {
    const MASK_DARK: [[u8; 3]; 3] = [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
    ];

    const MASK_GRAY: [[u8; 3]; 3] = [
            [0, 1, 0],
            [1, 1, 1],
            [0, 1, 0],
    ];

    const MASK_LIGHT: [[u8; 3]; 3] = [
            [0, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
    ];

    const MASK_WHITE: [[u8; 3]; 3] = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
    ];
    let img = Path::new("img.png");
    let img = open(img).expect("error img");
    let width = img.width();
    let height = img.height();
    println!("Открыта картинка: {:?}x{:?}", width, height);
    let rgb = img.to_rgb8();
    let rgb = rgb.as_raw();
    let res: Vec<u8> = rgb
        .par_chunks_exact(3)
        .map(|i| {
            let cb = (i[0] as f32 * 0.299) + (i[1] as f32 * 0.587) + (i[2] as f32 * 0.114);
            cb as u8
        })
        .collect();
    let path = Path::new("new.png");
    image::save_buffer(path, &res, width, height, image::ColorType::L8).expect("neydacha");
}
