use std::path::Path;
use std::env;

mod image_rect;
mod utils;

pub enum SortKey{
    Width,
    Height,
    MaxDimension
}

fn main() {

    let path = env::args();
    println!("path: {:?}", path.collect::<Vec<String>>());

    let mut images = match image_rect::image::get_images(&Path::new("./img/.")){
        Ok(imgs) => imgs,
        Err(_) => {
            println!("Error in getting images");
            Vec::new()
        }
    };

    utils::helpers::sort(&mut images, SortKey::MaxDimension);

    for i in images{
        println!("{:?}", i);
    }

}
