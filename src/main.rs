use std::env;
use image::GenericImageView;
use image::imageops::FilterType;

fn main() {
    // iterate args into image processing logic
    let args: Vec<String> = env::args().collect();
    if let None = args.get(1) {
        panic!("Hey! Drag some pictures into me to resize!");
    }

    for i in &args[1..] {
        // die while encountering unsupported formats
        let img = image::open(&i).unwrap();
        let width = img.dimensions().0;
        let height = img.dimensions().1;

        // determine whether pics are too small and inform the user
        if width < 512 && height < 512 {
            println!("This image \"{}\" seems to be too small. A poor result may be expected.", i);
        }

        // resize: select the longer side and resize to 512px, preserving ratio
        let ratio: f64 = (width / height) as f64;
        // 0: width; 1: height
        let mut longer_side: i32 = 0;
        if width < height {
            longer_side = 1;
        }
        match longer_side {
            0 => {
                let resized_width = 512;
                let resized_height = 512 as f64 * &ratio;
                img.resize(resized_width, resized_height as u32, FilterType::Nearest);
            }
            1 => {
                let resized_width = 512 as f64 * &ratio;
                let resized_height = 512;
                img.resize(resized_width as u32, resized_height, FilterType::Nearest);
            }
            _ => ()
        }
        img.save(i.to_owned() + ".resized.png").unwrap();
    }
}
