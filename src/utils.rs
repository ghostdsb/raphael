pub mod helpers{
    use std::cmp;
    use crate::image_rect;


    #[derive(Debug)]
    pub struct ImageHelper{
        pub size: u32,
        pub id: u32,
    }

    pub fn sort(list: &Vec<image_rect::image::ImageRect>) -> Vec<ImageHelper>{
        let image_size_list = image_list_builder(&list);
        image_size_list

    }

    fn image_list_builder(list: &Vec<image_rect::image::ImageRect>) -> Vec<ImageHelper>{
        list
        .into_iter()
        .map(|img| {ImageHelper{
            size: cmp::max(img.width, img.height),
            id: img.id
            }
        })
        .collect::<Vec<ImageHelper>>()     
    }

    // fn bin_sort<T>(list: Vec<T>)->Vec<T>{

    // }

    
}