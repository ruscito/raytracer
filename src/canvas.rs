use std::{cmp::min, ops::{Index, IndexMut}};


use crate::color::Color;
#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self  {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width*height],
        }
    }
    
    fn _pixel_idx(&self, x: usize, y: usize) -> usize {
        // Striding -encoding from matrix to a vector-
        // use row-first encoding; Each row of the grid will be stored
        // toghether, in x order. The next set of entries will contain
        // the second row and so on. A 5X2 will be stored as  
        // 0, 1, 2, 3, 4
        // 5, 6, 7, 8, 9
        (y * self.width) + x
    }

    pub fn backgound(&mut self, color: Color){
        for i in 0..self.pixels.len() {
            self.pixels[i] = color;
        }
    }

    pub fn to_raw_buffer(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        for pixel in &self.pixels {
            let (r, g, b) = pixel.to_bytes();
            out.push(r);
            out.push(g);
            out.push(b);
        }
        out
    }
    
    pub fn save(&self, file_name:&str) -> Result<(), String> {
        match image::save_buffer(
            file_name,
            &self.to_raw_buffer()[..], 
            self.width as u32, self.height as u32, 
            image::ColorType::Rgb8){
                Ok(_) => Ok(()),
                Err(_) => Err(String::from("Error saving the canvas"))
            }
    }

    fn _x_bound_check(&mut self, x:usize) -> usize {
        min(self.width - 1, x)        
    }
}

// https://stackoverflow.com/questions/33770989/implementing-the-index-operator-for-matrices-with-multiple-parameters
impl Index<[usize; 2]> for  Canvas {
    type Output = Color;
    fn index(&self, idx:[usize; 2]) -> &Self::Output {
        let mut y = self.height - idx[1];
        let x = min(self.width - 1, idx[0]); 
        y = min(self.height - 1, y); 
        &self.pixels[(y * self.width) + x]
    }    
}

impl IndexMut<[usize; 2]> for  Canvas {
    fn index_mut(&mut self, idx:[usize; 2]) -> &mut Self::Output {
        let mut y = self.height - idx[1];
        let x = min(self.width - 1, idx[0]); 
        y = min(self.height - 1, y); 
        &mut self.pixels[(y * self.width) + x]
    }    
}


impl Index<[f64; 2]> for  Canvas {
    type Output = Color;
    fn index(&self, idx:[f64; 2]) -> &Self::Output {
        let x:usize;
        let y:usize;

        if idx[0] < 0.0 {
            x = 0;
        } else {
            x = idx[0] as usize
        }

        if idx[1] < 0.0 {
            y = 0;
        } else {
            y = idx[1] as usize
        }
        
        &self[[x, y]]
    }    
}

impl IndexMut<[f64; 2]> for  Canvas {
    fn index_mut(&mut self, idx:[f64; 2]) -> &mut Self::Output {
        let x:usize;
        let y:usize;

        if idx[0] < 0.0 {
            x = 0;
        } else {
            x = idx[0] as usize
        }

        if idx[1] < 0.0 {
            y = 0;
        } else {
            y = idx[1] as usize
        }
        
        &mut self[[x, y]]
    }    
}

impl Index<[f32; 2]> for  Canvas {
    type Output = Color;
    fn index(&self, idx:[f32; 2]) -> &Self::Output {
        let x:usize;
        let y:usize;

        if idx[0] < 0.0 {
            x = 0;
        } else {
            x = idx[0] as usize
        }

        if idx[1] < 0.0 {
            y = 0;
        } else {
            y = idx[1] as usize
        }
        
        &self[[x, y]]
    }    
}

impl IndexMut<[f32; 2]> for  Canvas {
    fn index_mut(&mut self, idx:[f32; 2]) -> &mut Self::Output {
        let x:usize;
        let y:usize;

        if idx[0] < 0.0 {
            x = 0;
        } else {
            x = idx[0] as usize
        }

        if idx[1] < 0.0 {
            y = 0;
        } else {
            y = idx[1] as usize
        }
        
        &mut self[[x, y]]
    }    
}

