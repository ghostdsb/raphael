pub mod image{
    
    use image::{GenericImageView};
    use std::fs;
    use std::path::Path;
    use std::io;
    use std::cmp;

    #[derive(Debug)]
    pub struct ImageRect{
        pub width: u32,
        pub height: u32,
        pub max_dimension: u32,
        pub name: String,
        pub id: u32,
    }
    
    impl ImageRect{
        pub fn new(image_config: &ImageRect) -> Self{
            ImageRect{
                height: image_config.height,
                width: image_config.width,
                max_dimension: image_config.max_dimension,
                id: image_config.id,
                name: image_config.name.clone(),
            }
        }
    }
    
    pub fn get_images(dir: &Path) -> io::Result<Vec<ImageRect>> {
        let mut images: Vec<ImageRect> = Vec::new();
        let mut image_count = 0;
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let name = path.file_stem().unwrap();
                let name = name.to_str().unwrap();
                let name = String::from(name);
                let (width, height) = get_image_size(&path);
                let max_dimension =  cmp::max(height, width);
                let image = ImageRect::new(&ImageRect{width,height,name,max_dimension,
                    id: image_count});
                images.push(image);
                image_count += 1;
            }
        }
        Ok(images)
    }
    
    fn get_image_size(path: &Path) -> (u32, u32){
        let image = image::open(path).unwrap();
        image.dimensions()
    }
}
