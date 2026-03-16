use image::open;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

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
    let mut input = String::new();
    println!("Выберите формат вывода: 1 - Classic, 2 - RGB, 3 - 1В1 4 - Video");
    let choice = loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка ввода");
        if let Ok(num) = input.trim().parse::<u8>() {
            if num == 1 || num == 2 || num == 3 || num == 4 {
                break num;
            } else {
                println!("Введите от 1 до 4");
            }
        }
    };
    match choice {
        1 => {
            println!("Enter img path");
            let img = parse();
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
                                if STAMP_7[maty][matx] == 1 {
                                    row[x] = 0;
                                }
                            }
                            _ => (),
                        }
                    }
                });
            println!("Готово! Записываю");
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
            std::fs::write("output.txt", &buffer).expect("Не удалось записать файл");
            println!("Готово! Смотри результат в output.txt");
        }
        2 => {
            println!("Enter img path");
            let img = parse();
            let img = open(img).expect("error img");
            let width = img.width();
            let height = img.height();
            println!("Открыта картинка: {:?}x{:?}", width, height);
            let rgb = img.to_rgb8();

            let rgb = rgb.as_raw();
            let width = width * 5;
            let height = height * 5;
            let mut pngvec = vec![0; (width * height) as usize * 3];
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
        3 => {
            println!("Enter img path");
            let img = parse();
            let img = open(img).expect("error img");
            let width = img.width();
            let height = img.height();
            println!("Открыта картинка: {:?}x{:?}", width, height);
            let rgb = img.to_rgb8();

            let rgb = rgb.as_raw();
            println!("какой фон? 1.Черний 2.Автоматический");
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
            let colour = if choice == 2 { 2 } else { 255 };

            println!("введите кратность");
            let choice = loop {
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Ошибка ввода");
                if let Ok(num) = input.trim().parse::<usize>() {
                    if num > 0 {
                        break num;
                    } else {
                        println!("Введите больше 0");
                    }
                }
            };
            let kratnost = choice * 5;
            let skoka = choice;
            let mut pngvec = vec![0; (width * height) as usize * 3];
            pngvec
                .par_chunks_mut(width as usize * 3)
                .enumerate()
                .for_each(|(y, row)| {
                    let rowy = y / kratnost;
                    let maty = (y % kratnost) / skoka;
                    let rowy1 = rowy * kratnost;
                    for x in 0..width {
                        let pixel = x / kratnost as u32;
                        let pixel1 = pixel * kratnost as u32;
                        let matx = ((x % kratnost as u32) / skoka as u32) as usize;
                        let cords = ((rowy1 as u32 * width) + pixel1) * 3;
                        let cords = cords as usize;
                        let i = (x * 3) as usize;
                        if (rowy + pixel as usize) % 2 == 0 {
                            if STAMP_3[maty][matx] == 1 {
                                row[i] = rgb[cords]; // Переложили красный
                                row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2]; // Переложили синий
                            } else if STAMP_3[maty][matx] == 0 {
                                row[i] = rgb[cords] / colour; // Переложили красный
                                row[i + 1] = rgb[cords + 1] / colour; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2] / colour; // Переложили синий
                            }
                        } else {
                            if STAMP_7[maty][matx] == 1 {
                                row[i] = rgb[cords]; // Переложили красный
                                row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2]; // Переложили синий
                            } else if STAMP_7[maty][matx] == 0 {
                                row[i] = rgb[cords] / colour; // Переложили красный
                                row[i + 1] = rgb[cords + 1] / colour; // Переложили зеленый
                                row[i + 2] = rgb[cords + 2] / colour; // Переложили синий
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
        4 => {
            let _ = fs::create_dir_all("input_frames");
            let _ = fs::create_dir_all("output_frames");
            println!("Video name:");
            let input_video = parse();
            let extract_status = Command::new("ffmpeg")
                .arg("-y")
                .arg("-i")
                .arg(&input_video)
                .arg("input_frames/%04d.png")
                .status();
            match extract_status {
                Ok(status) if status.success() => println!("Видео нарезано успешно!"),
                _ => {
                    println!("Ошибка! ffmpeg не сработал. Он установлен в системе?");
                    return;
                }
            }
            let dir_entries = match fs::read_dir("input_frames") {
                Ok(entries) => entries,
                Err(e) => {
                    println!("Катастрофа! Не могу прочитать папку: {}", e);
                    return;
                }
            };
            let paths: Vec<_> = dir_entries
                .filter_map(|res| res.ok())
                .map(|entry| entry.path())
                .collect();
            if paths.is_empty() {
                println!("Папка пуста, кадров нет!");
                return;
            }
            println!("введите кратность");
            let choice = loop {
                input.clear();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Ошибка ввода");
                if let Ok(num) = input.trim().parse::<usize>() {
                    if num > 0 {
                        break num;
                    } else {
                        println!("Введите больше 0");
                    }
                }
            };
            paths.par_iter().for_each(|path| {
                let file_name = match path.file_name() {
                    Some(name) => name,
                    None => return,
                };
                let img = image::open(path).expect("error img").to_rgb8();
                let width = img.width();
                let height = img.height();
                let rgb = img.into_raw();
                let kratnost: usize = choice * 5;
                let skoka: usize = choice;
                let mut pngvec = vec![0; (width * height) as usize * 3];
                pngvec
                    .par_chunks_mut(width as usize * 3)
                    .enumerate()
                    .for_each(|(y, row)| {
                        let rowy = y / kratnost;
                        let maty = (y % kratnost) / skoka;
                        let rowy1 = rowy * kratnost;
                        for x in 0..width {
                            let pixel = x / kratnost as u32;
                            let pixel1 = pixel * kratnost as u32;
                            let matx = ((x % kratnost as u32) / skoka as u32) as usize;
                            let cords = ((rowy1 as u32 * width) + pixel1) * 3;
                            let cords = cords as usize;
                            let i = (x * 3) as usize;
                            if (rowy + pixel as usize) % 2 == 0 {
                                if STAMP_3[maty][matx] == 1 {
                                    row[i] = rgb[cords]; // Переложили красный
                                    row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                    row[i + 2] = rgb[cords + 2]; // Переложили синий
                                } else if STAMP_3[maty][matx] == 0 {
                                    row[i] = rgb[cords] / 2; // Переложили красный
                                    row[i + 1] = rgb[cords + 1] / 2; // Переложили зеленый
                                    row[i + 2] = rgb[cords + 2] / 2; // Переложили синий
                                }
                            } else {
                                if STAMP_7[maty][matx] == 1 {
                                    row[i] = rgb[cords]; // Переложили красный
                                    row[i + 1] = rgb[cords + 1]; // Переложили зеленый
                                    row[i + 2] = rgb[cords + 2]; // Переложили синий
                                } else if STAMP_7[maty][matx] == 0 {
                                    row[i] = rgb[cords] / 2; // Переложили красный
                                    row[i + 1] = rgb[cords + 1] / 2; // Переложили зеленый
                                    row[i + 2] = rgb[cords + 2] / 2; // Переложили синий
                                }
                            }
                        }
                    });
                let out_path = format!(
                    "output_frames/{}",
                    path.file_name().unwrap().to_str().unwrap()
                );
                image::save_buffer(out_path, &pngvec, width, height, image::ColorType::Rgb8)
                    .expect("Ошибка сохранения");
            });
            let fps_output = Command::new("ffprobe")
                .arg("-v")
                .arg("error") // Не пиши лишний мусор в консоль
                .arg("-select_streams")
                .arg("v:0") // Смотри только в видео-дорожку
                .arg("-show_entries")
                .arg("stream=r_frame_rate") // Дай мне только FPS
                .arg("-of")
                .arg("default=noprint_wrappers=1:nokey=1") // Выдай чистую цифру без текста
                .arg(&input_video) // Твой исходник
                .output() // Получаем результат
                .expect("Ошибка: ffprobe не сработал");
            let original_fps = String::from_utf8_lossy(&fps_output.stdout)
                .trim()
                .to_string();
            let build_status = Command::new("ffmpeg")
                .arg("-y") // Разрешаем перезаписать файл, если он уже существует
                .arg("-framerate")
                .arg(&original_fps) // Подставляем ту самую переменную с точным FPS!
                .arg("-i")
                .arg("output_frames/%04d.png") // Указываем папку с твоими готовыми 37-кадрами
                .arg("-i")
                .arg(&input_video) // Подкидываем оригинальное видео (чтобы забрать звук)
                .arg("-map")
                .arg("0:v:0") // Говорим: "Видеоряд бери из картинок"
                .arg("-map")
                .arg("1:a:0") // Говорим: "Аудиодорожку бери из оригинального видео"
                .arg("-c:v")
                .arg("libx264") // Кодируем стандартным кодеком, чтобы читалось везде
                .arg("-pix_fmt")
                .arg("yuv420p") // Фиксим цвета, чтобы не было черного экрана в плеерах
                .arg("-c:a")
                .arg("copy") // Звук копируем байт-в-байт, без потери качества!
                .arg("final_art37.mp4") // Название готового шедевра
                .status()
                .expect("Ошибка: ffmpeg крашнулся при склейке");

            if build_status.success() {
                println!("🔥 ГОТОВО! Видео final_art37.mp4 успешно собрано со звуком!");
            } else {
                println!("💀 Что-то пошло не так при сборке видео.");
            }
        }
        _ => (),
    }
}
