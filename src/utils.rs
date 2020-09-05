pub mod helpers{
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
            SortKey::Area => 
            list
            .sort_by(|img_a, img_b| img_b.area.cmp(&img_a.area)),
            SortKey::MaxDimension =>
            list
            .sort_by(|img_a, img_b| img_b.max_dimension.cmp(&img_a.max_dimension))
        }

    }
    
}