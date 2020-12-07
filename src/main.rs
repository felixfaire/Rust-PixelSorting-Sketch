use std::io;
use std::path::Path;

mod pixel_sort;
mod area_glitch;

fn main() {
  
  let mut filename = String::new();
  let mut vertical = String::new();

  println!("Type relative filename");
  io::stdin().read_line(&mut filename).expect("Failed to read filename");
  filename = filename.trim().to_owned();

  println!("Vertical? y / n");
  io::stdin().read_line(&mut vertical).expect("Failed to read line");

  let vertical = match vertical.to_lowercase().trim() {
    "y" => true,
    "n" => false,
    _ => false
  };

  
  multi_output_pixelsort(filename, 10, 150, 200, vertical);
  

}


fn multi_output_pixelsort(filename: String, num_iterations: u32, min: u8, max: u8, vertical: bool) {

  let max = max as u32;
  let min = min as u32;
  let filestem = Path::new(&filename);
  let filestem = filestem.file_stem().unwrap().to_str().unwrap();

  let filepath = "input/".to_owned() + &filename;
  println!("Sorting file {} {} times", filepath, num_iterations);

  let mut img = image::open(filepath).unwrap();

  if vertical {
    img = img.rotate90();
  }

  let step: u32 = (max - min) / (num_iterations + 1);
  println!("{}", step);
  let mut i: u32 = min + step;

  while i < max {
    println!("Calculating image at threshold: {}", i);
    
    let thresh = i as u8;
    let mut new_img = img.clone();

    area_glitch::shuffle_areas(&mut new_img, 5);
    pixel_sort::sort_pixels(&mut new_img, thresh);
    
    if vertical {
      new_img = new_img.rotate270();
    }
    
    let out_path = "output/".to_owned() + filestem + "_" + &thresh.to_string() + ".png";
    new_img.save(out_path).unwrap();

    i += step;
  }
}