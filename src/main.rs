extern crate termi_graphics;
use termi_graphics::pixel_art::{PixelColors, Screen};

// Any live cell with fewer than two live neighbors dies, as if caused by under population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

fn main() {
    let mut shape1 = Vec::new();

    //smily face
    shape1.push(vec![2, 2, 0, 2, 2]);
    shape1.push(vec![2, 2, 0, 2, 2]);
    shape1.push(vec![0, 0, 0, 0, 0]);
    shape1.push(vec![0, 0, 1, 0, 0]);
    shape1.push(vec![8, 0, 0, 0, 8]);
    shape1.push(vec![8, 8, 8, 8, 8]);
    let mut screen1 = Screen::new(8, 8, PixelColors::Black).unwrap();
    screen1.attach_pixels(&shape1, 1, 1).unwrap();
    //simply print it once:
    screen1.print_screen();
    // println!("Hello, world!");
}
