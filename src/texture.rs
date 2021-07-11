use image::GenericImageView;
use crate::color::Color;
use crate::math::FloatVector;

use std::path::Path;

pub trait Texture {
    fn get(&self, u: f32, v: f32) -> Color;
}

pub struct ConstantTexture {
    color: Color,
}

impl Texture for ConstantTexture {
    fn get(&self, _u: f32, _v: f32) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    color_a: Color,
    color_b: Color,
    scale: f32,
}

impl Texture for CheckerTexture {
    fn get(&self, u: f32, v: f32) -> Color {
        let value = (u * self.scale).sin() * (v * self.scale).sin();
        if value < 0.0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

pub struct ImageTexture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl ImageTexture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;
        let width = image.dimensions().0 as usize;
        let height = image.dimensions().1 as usize;

        let mut pixels = vec![Color::zero(); width * height];

        for (x, y, p) in image.as_rgb8().unwrap().enumerate_pixels() {
            let pixel = &mut pixels[x as usize + y as usize * width];
            pixel.x = p[0] as f32 / 255.0;
            pixel.y = p[1] as f32 / 255.0;
            pixel.z = p[2] as f32 / 255.0;
            pixel.w = 1.0;
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }
}

impl Texture for ImageTexture {
    fn get(&self, u: f32, v: f32) -> Color {
        let x = (u * self.width as f32) as usize;
        let y = (v * self.height as f32) as usize;

        self.pixels[x + y * self.width]
    }
}
