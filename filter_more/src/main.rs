mod cli;
mod bmp;

use bmp::{read_bmp, write_bmp, RGBTriple};
use cli::{CommandOptions, CliFlag};

fn main() {
    let options = match CommandOptions::build() {
        Ok(opt) => opt,
        Err(message) => {
            println!("{message}");
            return
        }
    };

    let (bmp_header, bmp_info, mut rgb_triple) = read_bmp(&options.source).unwrap();

    match options.flag {
        CliFlag::Blur => { blur(&mut rgb_triple); }
        CliFlag::Edges => { println!("todo!"); }
        CliFlag::GreyScale => { grey_scale(&mut rgb_triple); }
        CliFlag::Reflect => { println!("todo!"); }
    };

    let _ = write_bmp(&options.output, &bmp_header, &bmp_info, &rgb_triple).unwrap();
}

// For the purposes of the exercise, we'll define these contracts to edit the bitmap in-place.
// We know that might lead to some inefficencies without more complicated work...
fn blur(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    let copy = rgb_triple.clone(); // we know, not great...
    let m = copy.len();
    let n = copy[0].len();

    for row in 0..m {
        for col in 0..n {
            let mut sums = [0u16; 3];
            let mut count = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let r = row as i32 + dr;
                    let c = col as i32  + dc;
                    if r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
                        count += 1;
                        sums[0] += copy[r as usize][c as usize].rgb_blue as u16; // u16 should be able to fit any u8 * 9...
                        sums[1] += copy[r as usize][c as usize].rgb_green as u16;
                        sums[2] += copy[r as usize][c as usize].rgb_red as u16;
                    }
                }
            }
            
            rgb_triple[row][col].rgb_blue = (sums[0] / count) as u8;
            rgb_triple[row][col].rgb_green = (sums[1] / count) as u8;
            rgb_triple[row][col].rgb_red = (sums[2] / count) as u8;
        }
    }
}

// For the purposes of the exercise, we'll define these contracts to edit the bitmap in-place.
// We know that might lead to some inefficencies without more complicated work...
fn grey_scale(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    let m = rgb_triple.len();
    let n = rgb_triple[0].len();

    for row in 0..m {
        for col in 0..n {
            let r = rgb_triple[row][col].rgb_red as u16;
            let g = rgb_triple[row][col].rgb_green as u16;
            let b = rgb_triple[row][col].rgb_blue as u16;
            let average = (r + g + b) / 3; 
            rgb_triple[row][col].rgb_blue = average as u8;
            rgb_triple[row][col].rgb_green = average as u8;
            rgb_triple[row][col].rgb_red = average as u8;
        }
    }
}