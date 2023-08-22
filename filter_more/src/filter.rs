
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
            
            rgb_triple[row][col].rgb_blue = (sums[0] as f32 / count as f32).round() as u8;
            rgb_triple[row][col].rgb_green = (sums[1] as f32 / count as f32).round() as u8;
            rgb_triple[row][col].rgb_red = (sums[2] as f32 / count as f32).round() as u8;
        }
    }
}

pub fn edges(rgb_triple: &mut Vec<Vec<RGBTriple>>) {
    let copy = rgb_triple.clone();
    let m = rgb_triple.len();
    let n = rgb_triple[0].len();

    let sobel_gx: [[i32; 3]; 3] = [
        [-1, 0 ,1],
        [-2, 0, 2],
        [-1, 0, 1]
    ];

    let sobel_gy: [[i32; 3]; 3] = [
        [-1, -2 ,-1],
        [0, 0, 0],
        [1, 2, 1]
    ];

    for row in 0..m {
        for col in 0..n {
            let mut gx_sums = [0i32; 3];
            let mut gy_sums = [0i32; 3];
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let r = row as i32 + dr;
                    let c = col as i32  + dc;
                    if r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
                        let dr_as_idx = (dr + 1) as usize;
                        let dc_as_idx = (dc + 1) as usize;
                        gx_sums[0] += copy[r as usize][c as usize].rgb_blue as i32 * sobel_gx[dr_as_idx][dc_as_idx];
                        gx_sums[1] += copy[r as usize][c as usize].rgb_green as i32 * sobel_gx[dr_as_idx][dc_as_idx];
                        gx_sums[2] += copy[r as usize][c as usize].rgb_red as i32 * sobel_gx[dr_as_idx][dc_as_idx];
                        gy_sums[0] += copy[r as usize][c as usize].rgb_blue as i32 * sobel_gy[dr_as_idx][dc_as_idx];
                        gy_sums[1] += copy[r as usize][c as usize].rgb_green as i32 * sobel_gy[dr_as_idx][dc_as_idx];
                        gy_sums[2] += copy[r as usize][c as usize].rgb_red as i32 * sobel_gy[dr_as_idx][dc_as_idx];
                    }
                }
            }
            
            
            rgb_triple[row][col].rgb_blue = (gx_sums[0].pow(2) as f32 + gy_sums[0].pow(2) as f32).sqrt().round() as u8;
            rgb_triple[row][col].rgb_green = (gx_sums[1].pow(2) as f32 + gy_sums[1].pow(2) as f32).sqrt().round() as u8;
            rgb_triple[row][col].rgb_red = (gx_sums[2].pow(2) as f32 + gy_sums[2].pow(2) as f32).sqrt().round() as u8;
        }
    }
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