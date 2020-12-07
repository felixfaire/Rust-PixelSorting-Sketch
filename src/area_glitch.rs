
use rand;
use image::{DynamicImage, GenericImageView, GenericImage};

#[derive(Debug)]
struct Area {
  x: u32,
  y: u32,
  w: u32,
  h: u32
}

impl Area {
  fn rand_new(dims: (u32, u32)) -> Area {
    let mut area = Area {
      x: (rand::random::<f32>() * dims.0 as f32) as u32,
      y: (rand::random::<f32>() * dims.1 as f32) as u32,
      w: ((rand::random::<f32>() * 0.05 + 0.8) * dims.0 as f32) as u32,
      h: ((rand::random::<f32>() * 0.05 + 0.8) * dims.1 as f32) as u32,
    };

    if (area.x + area.w) > dims.0 {
      area.w -= (area.x + area.w) - dims.0;
    }

    if (area.y + area.h) > dims.1 {
      area.y -= (area.y + area.h) - dims.1;
    }

    area
  }
}

/**
 * This function cuts out and rearranges various rectangles of pixels (of different sizes) within the image
 */
pub fn shuffle_areas(img: &mut DynamicImage, num_times: u32) {

  let dims = img.dimensions();

  for _i in 0..num_times {
    
    let mut area = Area::rand_new(dims);
    let crop = img.crop_imm(area.x, area.y, area.w, area.h);

    area.x = (rand::random::<f32>() * dims.0 as f32) as u32;
    area.y = (rand::random::<f32>() * dims.0 as f32) as u32;

    for src_x in 0..area.h {
      for src_y in 0..area.w {
        
        let x = area.x + src_y;
        let y = area.y + src_x; 

        if x < dims.0 && y < dims.1 {
          img.put_pixel(x, y, crop.get_pixel(src_y, src_x));
        }
      }
    }
  }
}