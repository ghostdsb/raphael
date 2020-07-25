pub mod helpers{
    use std::cmp;
    use crate::image_rect;
    use crate::SortKey;


    #[derive(Debug)]
    pub struct ImageHelper{
        pub size: u32,
        pub id: u32,
    }

    pub fn sort(list: &mut Vec<image_rect::image::ImageRect>, key: SortKey){

        match key{
            SortKey::Height =>
            list
            .sort_by(|img_a, img_b| img_b.height.cmp(&img_a.height)),
            SortKey::Width => 
            list
            .sort_by(|img_a, img_b| img_b.width.cmp(&img_a.width)),
            SortKey::MaxDimension =>
            list
            .sort_by(|img_a, img_b| img_b.max_dimension.cmp(&img_a.max_dimension))
        }

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