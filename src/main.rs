use std::{env, process};
use std::io::{stdin, stdout, Read, Write};
use image::GenericImageView;
use image::imageops::FilterType;
use indicatif::ProgressBar;

fn pseudo_pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress [Enter] to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn graceful_shutdown(code: i32) {
    pseudo_pause();
    process::exit(code);
}

fn main() {
    // iterate args into image processing logic
    let args: Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Hey! Drag some pictures into me to resize!");
        graceful_shutdown(1);
    }

    let bar = ProgressBar::new(args.len() as u64 - 1);

    let mut too_small_exist = false;

    for i in &args[1..] {
        bar.inc(1);

        let img = image::open(&i);
        let img = match img {
            Ok(file) => file,
            Err(_) => {
                ProgressBar::println(&bar, format!("ERROR: Unsupported or broken file: {}", i));
                continue;
            }
        };
        let width = img.dimensions().0;
        let height = img.dimensions().1;

        // determine whether pics are too small and inform the user
        if width < 512 && height < 512 {
            too_small_exist = true;
            ProgressBar::println(&bar, format!("This image \"{}\" is too small. A poor result may be expected.", i));
        }

        // resize: select the longer side and resize to 512px, preserving ratio
        let ratio = width as f64 / height as f64;
        match width > height {
            true => {
                let nwidth = 512;
                let nheight = (512 as f64 * ratio) as u32 + 1;
                let nimg = img.resize(nwidth, nheight, FilterType::Lanczos3);
                nimg.save(format!("{}.resized.png", i)).unwrap();
            }
            false => {
                let nwidth = (512 as f64 * ratio) as u32 + 1;
                let nheight = 512;
                let nimg = img.resize(nwidth, nheight, FilterType::Lanczos3);
                nimg.save(format!("{}.resized.png", i)).unwrap();
            }
        }
    }

    bar.finish();
    println!("All done! Have fun!");
    if too_small_exist == true {
        println!("HINT: You can use upscaling tools such as waifu2x on your pictures before dragging them into resizer.");
    }
    graceful_shutdown(0);
}
