
use crate::bmp::{RGBTriple};
// For the purposes of the exercise, we'll define these contracts to edit the bitmap in-place.
// We know that might lead to some inefficencies without more complicated work...
pub fn blur(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
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

pub fn edges(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    todo!();
}

// For the purposes of the exercise, we'll define these contracts to edit the bitmap in-place.
// We know that might lead to some inefficencies without more complicated work...
pub fn grey_scale(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    let m = rgb_triple.len();
    let n = rgb_triple[0].len();

    for row in 0..m {
        for col in 0..n {
            let r = rgb_triple[row][col].rgb_red as f32;
            let g = rgb_triple[row][col].rgb_green as f32;
            let b = rgb_triple[row][col].rgb_blue as f32;
            let average = ((r + g + b) / 3.0).round(); 
            rgb_triple[row][col].rgb_blue = average as u8;
            rgb_triple[row][col].rgb_green = average as u8;
            rgb_triple[row][col].rgb_red = average as u8;
        }
    }
}

pub fn reflect(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    for row in rgb_triple {
        row.reverse();
    }
}