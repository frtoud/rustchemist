extern crate image;
extern crate glium;

use glium::texture::Texture2d;

pub fn get_sprite(display: &glium::Display, filepath: &str) -> Texture2d
{
    let mut string: String = "./content/sprites/".to_owned();
    string.push_str(filepath);

    let file = image::open(string).unwrap().to_rgba();
    let dimensions = file.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&file.into_raw(), dimensions);

    //Debug info
    println!("loaded {}, {:?}", filepath, dimensions);
    Texture2d::new(display, image).unwrap()
}