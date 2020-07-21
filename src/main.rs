extern crate image;

use image::{GenericImageView};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;

fn main() {
    // let img = image::open("img/ic_prize.png").unwrap();
    // println!("dimensions: {:?}",img.dimensions());
    // img.save("test.png").unwrap();
    
    // for entry in fs::read_dir("."){
    //     println!("entry: {:?}", entry);
    //     // let path = entry.path();
    // }

    visit_dirs(&Path::new("./img/.")).unwrap()

}

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("-- {:?}",path);
            print_image_size(&path);
        }
    }
    Ok(())
}

fn print_image_size(path: &Path){
    let image = image::open(path).unwrap();
    println!("dimensions: {:?}", image.dimensions());
}