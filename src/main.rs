use std::path::Path;
use std::env;

mod image_rect;
mod utils;

pub enum SortKey{
    Width,
    Height,
    Area,
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

    // let mut ids: Vec<u32> = Vec::new();
    // for i in images{
    //     ids.push(i.id);
    // }
    // println!("{:?}", ids);

    for i in &images{
        println!("{:?}", i);
    }

    let mut image_tree = image_rect::image::ImageTree::new(0,0,1280,1280);

    for image in images{
        let img = image.clone();
        let image_node = image_rect::image::ImageNode::new(0,0,img.width, img.height);
        image_tree.insert(image_node);
    }

    println!("tree: {:?}", image_tree);

}
