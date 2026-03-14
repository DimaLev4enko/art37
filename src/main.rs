use image::open;
use rayon::prelude::*;
use std::path::Path;

const MASK_DARK: [[u8; 3]; 3] = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];

const MASK_GRAY: [[u8; 3]; 3] = [[0, 1, 0], [1, 1, 1], [0, 1, 0]];

const MASK_LIGHT: [[u8; 3]; 3] = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];

const MASK_WHITE: [[u8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
const STAMP_3: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1], // y=0 (Верх)
    [0, 0, 0, 0, 1], // y=1
    [1, 1, 1, 1, 1], // y=2 (Середина)
    [0, 0, 0, 0, 1], // y=3
    [1, 1, 1, 1, 1], // y=4 (Низ)
];
const STAMP_7: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1], // y=0 (Крыша)
    [0, 0, 0, 0, 1], // y=1
    [0, 0, 0, 1, 0], // y=2
    [0, 0, 1, 0, 0], // y=3
    [0, 1, 0, 0, 0], // y=4 (Ножка)
];
fn parse() -> String {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).expect("cant parse");
        if buffer.is_empty() {
            println!("Try again");
        } else {
            break buffer.trim().to_string();
        }
    }
}
fn main() {
    println!("Enter img path");
    let img = parse();
    let img = open(img).expect("error img");
    let width = img.width();
    let height = img.height();
    println!("Открыта картинка: {:?}x{:?}", width, height);
    let rgb = img.to_rgb8();
    let mut input = String::new();
    println!("Выберите формат вывода: 1 - Classic, 2 - RGB");
    let choice = loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка ввода");
        if let Ok(num) = input.trim().parse::<u8>() {
            if num == 1 || num == 2 {
                break num;
            } else {
                println!("Введите 1 или 2");
            }
        }
    };
    let rgb = rgb.as_raw();
    match choice {
        1 => {
            let res: Vec<u8> = rgb
                .par_chunks_exact(3)
                .map(|i| {
                    let cb = (i[0] as f32 * 0.299) + (i[1] as f32 * 0.587) + (i[2] as f32 * 0.114);
                    cb as u8
                })
                .collect();
            // let path = Path::new("new.png");
            // image::save_buffer(path, &res, width, height, image::ColorType::L8).expect("neydacha");
            let width = ((width * 3) + 1) as usize;
            let height = (height * 3) as usize;
            let mut buffer = vec![b' '; width * height];
            buffer
                .par_chunks_mut(width)
                .enumerate()
                .for_each(|(y, row)| {
                    let rowy = y / 3;
                    let maty = y % 3;
                    let mut currtect = b'3';
                    for x in 0..(width - 1) {
                        let pixel = x / 3;
                        let matx = x % 3;
                        let cords = (rowy * (width / 3)) + pixel;
                        match res[cords] {
                            0..64 => {
                                if MASK_DARK[maty][matx] == 1 {
                                    row[x] = currtect;
                                    if currtect == b'3' {
                                        currtect = b'7';
                                    } else if currtect == b'7' {
                                        currtect = b'3';
                                    }
                                }
                            }
                            64..128 => {
                                if MASK_GRAY[maty][matx] == 1 {
                                    row[x] = currtect;
                                    if currtect == b'3' {
                                        currtect = b'7';
                                    } else if currtect == b'7' {
                                        currtect = b'3';
                                    }
                                }
                            }
                            128..192 => {
                                if MASK_LIGHT[maty][matx] == 1 {
                                    row[x] = currtect;
                                    if currtect == b'3' {
                                        currtect = b'7';
                                    } else if currtect == b'7' {
                                        currtect = b'3';
                                    }
                                }
                            }
                            192..=255 => {
                                if MASK_WHITE[maty][matx] == 1 {
                                    row[x] = currtect;
                                    if currtect == b'3' {
                                        currtect = b'7';
                                    } else if currtect == b'7' {
                                        currtect = b'3';
                                    }
                                }
                            }
                        }
                    }
                    row[width - 1] = b'\n';
                });
            drop(res);
            let width = width * 5;
            let height = height * 5;
            let mut pngvec = vec![u8::MAX; width * height];
            pngvec
                .par_chunks_mut(width)
                .enumerate()
                .for_each(|(y, row)| {
                    let rowy = y / 5;
                    let maty = y % 5;
                    for x in 0..width {
                        let pixel = x / 5;
                        let matx = x % 5;
                        let cords = (rowy * (width / 5)) + pixel;
                        match buffer[cords] {
                            51 => {
                                if STAMP_3[maty][matx] == 1 {
                                    row[x] = 0;
                                }
                            }
                            55 => {
                                if STAMP_3[maty][matx] == 1 {
                                    row[x] = 0;
                                }
                            }
                            _ => (),
                        }
                    }
                });
            println!("Готово! Записываю");
            println!("Выберите формат вывода: 1 - BMP, 2 - PNG");
            let choice = loop {
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Ошибка ввода");
                if let Ok(num) = input.trim().parse::<u8>() {
                    if num == 1 || num == 2 {
                        break num;
                    } else {
                        println!("Введите 1 или 2");
                    }
                }
            };
            match choice {
                1 => {
                    println!("Enter file name");
                    let path = parse();
                    image::save_buffer(
                        path,
                        &pngvec,
                        width as u32,
                        height as u32,
                        image::ColorType::L8,
                    )
                    .expect("neydacha");
                }
                2 => {
                    println!("Enter file name");
                    let path = parse();
                    image::save_buffer(
                        path,
                        &pngvec,
                        width as u32,
                        height as u32,
                        image::ColorType::L8,
                    )
                    .expect("neydacha");
                }
                _ => unreachable!(),
            }
            std::fs::write("output.txt", &buffer).expect("Не удалось записать файл");
            println!("Готово! Смотри результат в output.txt");
        }
        2 => {
            let width = width * 5;
            let height = height * 5;
            let mut pngvec = vec![128; (width * height) as usize * 3];
            pngvec
                .par_chunks_mut(width as usize * 3)
                .enumerate()
                .for_each(|(y, row)| {
                    let rowy = y / 5;
                    let maty = y % 5;
                    for x in 0..width {
                        let pixel = x / 5;
                        let matx = (x % 5) as usize;
                        let cords = ((rowy as u32 * (width / 5)) + pixel) * 3;
                        let cords = cords as usize;
                        let i = (x * 3) as usize;
                        if (rowy + pixel as usize) % 2 == 0 {
                            if STAMP_3[maty][matx] == 1 {
                                row[i] = rgb[cords]; // Переложили красный
                                row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2]; // Переложили синий
                            }
                        } else {
                            if STAMP_7[maty][matx] == 1 {
                                row[i] = rgb[cords]; // Переложили красный
                                row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2]; // Переложили синий
                            }
                        }
                    }
                });

            println!("Enter file name");
            let path = parse();
            image::save_buffer(
                path,
                &pngvec,
                width as u32,
                height as u32,
                image::ColorType::Rgb8,
            )
            .expect("neydacha");
        }
        _ => (),
    }
}
