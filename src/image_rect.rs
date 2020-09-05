pub mod image{
    
    use image::{GenericImageView};
    use std::fs;
    use std::path::Path;
    use std::io;
    use std::cmp;

    #[derive(Debug, Clone)]
    pub struct ImageRect{
        pub width: u32,
        pub height: u32,
        pub area: u32,
        pub max_dimension: u32,
        pub name: String,
        pub id: u32,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct ImageNode{
        pub x: u32,
        pub y: u32,
        pub w: u32,
        pub h: u32,
        pub used: bool,
    }

    #[derive(Debug, Clone)]
    pub struct ImageTree{
        pub image: ImageNode,
        pub down: Option<Box<ImageTree>>,
        pub right: Option<Box<ImageTree>>
    }

    impl ImageNode{
        pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self{
            ImageNode{
                x,y,w,h,
                used: false,
            }
        }
    }

    impl ImageTree{

        pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self{
            ImageTree{
                image: ImageNode::new(x,y,w,h),
                down: None,
                right: None
            }
        } 

        pub fn insert(&mut self, image: ImageNode){
            if self.image.used == false{
                self.image = image;
                self.image.used = true;
                self.down = Some(Box::new(ImageTree::new(self.image.x, self.image.y + image.h, self.image.w, self.image.h-image.h)));
                self.right = Some(Box::new(ImageTree::new(self.image.x + self.image.w, self.image.y , self.image.w - image.w, image.h)));
            }else{
                let down_node = self.down.as_ref().unwrap();
                // let right_node = self.right.as_ref().unwrap();
                
                if down_node.image.w > image.w{
                    match self.down{
                        Some(ref mut node) => node.insert(image),
                        None => ()
                    }
                }else{
                    match self.right{
                        Some(ref mut node) => node.insert(image),
                        None => ()
                    }
                }
                
            }
        }
    }
    
    impl ImageRect{
        pub fn new(image_config: &ImageRect) -> Self{
            ImageRect{
                height: image_config.height,
                width: image_config.width,
                max_dimension: image_config.max_dimension,
                area: image_config.area,
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
                let area = width * height;
                let image = ImageRect::new(&ImageRect{width,height,name,max_dimension,area,
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
