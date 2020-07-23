use std::path::Path;
use std::env;

mod image_rect;
mod utils;

fn main() {

    let path = env::args();
    println!("path: {:?}", path.collect::<Vec<String>>());

    let images = match image_rect::image::get_images(&Path::new("./img/.")){
        Ok(imgs) => imgs,
        Err(_) => {
            println!("Error in getting images");
            Vec::new()
        }
    };

    let images_ = utils::helpers::sort(&images);

    for i in images{
        println!("{:?}", i);
    }

}
