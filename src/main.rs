use image::{GenericImageView, ImageBuffer, RgbaImage, Rgba};
use std::{time::Instant};

#[derive(Debug, Clone)]
struct SortElement {
    luminance: f32,
    pixel_info: Vec<u8>,
}

/*fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/

fn convert_image_to_vector(dir: &str) -> Vec<SortElement> {
    //println!("{:?}", img.dimensions());
    let img = image::open(dir).unwrap();
    let (mut r, mut g, mut b): (u8, u8, u8);
    let mut pixel_luminance: f32;
    let mut rgba_vector: Vec<u8>;

    let mut sorting_vector: Vec<SortElement> = Vec::new();


    for pixel in img.pixels() {
        
        r = pixel.2.0[0];
        g = pixel.2.0[1];
        b = pixel.2.0[2]; 
        
        pixel_luminance = 0.2126*r as f32 + 0.7152*g as f32 + 0.0722*b as f32;

        rgba_vector = pixel.2.0.to_vec();



        let element = SortElement {
            luminance: pixel_luminance,
            pixel_info: rgba_vector,
        };
        
        sorting_vector.push(element);
    }

    return sorting_vector; 
}



fn get_image_dimensions(dir: &str) -> (u32, u32) { 
    let img = image::open(dir).unwrap();
    return img.dimensions();
}

fn save_image(dimensions: (u32, u32), image_vector: Vec<SortElement>, _name: String) {
    let image_width: u32 = dimensions.0;
    let image_height: u32 = dimensions.1;
    let mut current_index: u32 = 0;

    

    let mut buffer: RgbaImage = ImageBuffer::new(image_width, image_height);

    for(_x, _y, pixel) in buffer.enumerate_pixels_mut(){
        //println!("{:?}", image_vector[currentIndex as usize].pixel_info);
        let (r, g, b, a): (u8, u8, u8, u8);

        r = image_vector[current_index as usize].pixel_info[0];
        g = image_vector[current_index as usize].pixel_info[1];
        b = image_vector[current_index as usize].pixel_info[2];
        //g = (image_vector[currentIndex as usize].pixel_info[1] as f32 - ((image_vector[currentIndex as usize].pixel_info[1]) as f32)) as u8;
        //print_type_of(&image_vector[currentIndex as usize].pixel_info);
        a = image_vector[current_index as usize].pixel_info[3];
        //b = (image_vector[currentIndex as usize].pixel_info[2] as f32 - ((image_vector[currentIndex as usize].pixel_info[2]) as f32)) as u8;

        *pixel = Rgba([r,g,b,a]);
        // println!("{:?}", pixel);
        current_index += 1;
    }

    let filename = _name + ".png";
    match buffer.save(filename) {
        Err(e) => println!("Error writing file: {}", e),
        Ok(()) => println!("done!"),
    }
}


fn sort_image_by_luminosity(image_vector: Vec<SortElement>, _mask_vector: Vec<SortElement>) -> Vec<SortElement> {
    let mut vector_to_sort: Vec<SortElement> = image_vector.to_vec();
    let mut vector_mask: Vec<SortElement> = _mask_vector.to_vec();
    let mut start: usize = 0 as usize;
    let mut second_iterator: usize;
    let mut minimal_lumi: f32;
    let mut swap_cache: SortElement;

    let mut lumi_sum: f32 = 0.0;

    for pixel in image_vector {
        lumi_sum += pixel.luminance as f32;
    }

    // println!("{}", lumi_sum/vector_to_sort.len() as f32);
    //FIXME: replace the insertion sort algorithm with something quicker (eg. quicksort, merge sort);
    
    /*for pixel in image_vector {
    }*/
    
    while start < vector_to_sort.len() { 
        // if vector_to_sort[start].luminance > (lumi_sum/vector_to_sort.len() as f32) as f32 {
        minimal_lumi = vector_to_sort[start].luminance;
        second_iterator = start;
        while second_iterator < vector_to_sort.len() {
            if vector_to_sort[second_iterator].luminance > minimal_lumi && vector_mask[second_iterator].pixel_info == vec![255,255,255,255]  {
                minimal_lumi = vector_to_sort[start].luminance;
                swap_cache = vector_to_sort[start].clone();
                vector_to_sort[start] = vector_to_sort[second_iterator].clone();
                vector_to_sort[second_iterator] = swap_cache;
            }
            second_iterator += 1;
            }
        // }
        start += 1;
    }

    return vector_to_sort;

}

fn create_contrast_mask(_image_vector: &Vec<SortElement>, _low: f32, _high: f32, _dimensions: &(u32, u32)) -> Vec<SortElement> {
    let mut mask_vector: Vec<SortElement> = _image_vector.to_vec();
    let mut masked: Vec<SortElement> = vec![];
    for mut pixel in &mask_vector[..] {
        // println!("{:?}", pixel);
        if pixel.luminance > _high || pixel.luminance < _low {
            masked.push(SortElement{luminance: pixel.luminance, pixel_info: vec![0,0,0,255]});
            // pixel.pixel_info = vec![0,0,0,1].to_vec();
            continue;
        }

        masked.push(SortElement{luminance: pixel.luminance, pixel_info: vec![255,255,255,255]});
    }

    save_image(*_dimensions, masked.clone(), "contrast-mask".to_string());
    return masked;
}


fn main() {
    let image_path: &str = "image-2.png";
    let before = Instant::now();
    let mut image_vector: Vec<SortElement> = convert_image_to_vector(&image_path);
    let mask_vector: Vec<SortElement> = create_contrast_mask(&image_vector,80.0, 150.0, &get_image_dimensions(image_path));
    image_vector = sort_image_by_luminosity(image_vector.clone(), mask_vector);
    save_image(get_image_dimensions(image_path), image_vector, "image".to_string());
    
    println!("Elapsed time: {:.2?}", before.elapsed());
}


//TODO: use the contrast mask to only sort specific pixels!