// Purpose: Library for filter_more
use std::fs::File;
use std::io::{self, Read, SeekFrom, Seek, Write};

#[repr(C, packed)]
pub struct BITMAPFILEHEADER {
    bf_type: u16,
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    bf_off_bits: u32,
}

#[repr(C, packed)]
pub struct BITMAPINFOHEADER {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

pub struct RGBTriple {
    rgb_blue: u8,
    rgb_green: u8,
    rgb_red: u8,
}


impl Default for BITMAPFILEHEADER {
    fn default() -> Self {
        BITMAPFILEHEADER {
            bf_type: 0,
            bf_size: 0,
            bf_reserved1: 0,
            bf_reserved2: 0,
            bf_off_bits: 0,
        }
    }
}

impl Default for BITMAPINFOHEADER {
    fn default() -> Self {
        BITMAPINFOHEADER {
            bi_size: 0,
            bi_width: 0,
            bi_height: 0,
            bi_planes: 0,
            bi_bit_count: 0,
            bi_compression: 0,
            bi_size_image: 0,
            bi_x_pels_per_meter: 0,
            bi_y_pels_per_meter: 0,
            bi_clr_used: 0,
            bi_clr_important: 0,
        }
    }
}

impl RGBTriple {
    pub fn new() -> Self {
        RGBTriple {
            rgb_blue: 0,
            rgb_green: 0,
            rgb_red: 0,
        }
    }

    // Byte array's len guarenteed to be 3;
    pub fn from_u8_bytes(bytes: &[u8]) -> Self {
        RGBTriple {
            rgb_blue: bytes[0],
            rgb_green: bytes[1],
            rgb_red: bytes[2],
        }
    }

    pub fn to_u8_bytes(&self) -> [u8; 3] {
        [self.rgb_blue, self.rgb_green, self.rgb_red]
    }
}

pub fn read_in_struct<T: Default + Sized, R: Read>(reader: &mut R) -> io::Result<T> {
    let mut t = T::default();
    reader.read_exact(unsafe {
        std::slice::from_raw_parts_mut(
            &mut t as *mut T as *mut u8,
            std::mem::size_of::<T>(),
        )
    })?;
    Ok(t)
}

pub fn write_out_struct<T: Sized, W: std::io::Write>(t: &T, mut writer: W) -> io::Result<()> {
    writer.write_all(unsafe {
        std::slice::from_raw_parts(
            t as *const T as *const u8,
            std::mem::size_of::<T>(),
        )
    })?;
    Ok(())
}



pub fn read_bmp(filename: &str) -> io::Result<(BITMAPFILEHEADER, BITMAPINFOHEADER, Vec<Vec<RGBTriple>>)> {
    let mut file = File::open(filename)?;
    let bf: BITMAPFILEHEADER = read_in_struct(&mut file)?;
    let bi: BITMAPINFOHEADER = read_in_struct(&mut file)?;

    // Ensure infile is (likely) a 24-bit uncompressed BMP 4.0 - taken from filter_more impl...
    if bf.bf_type != 0x4d42 || bf.bf_off_bits != 54 || bi.bi_size != 40 ||
        bi.bi_bit_count != 24 || bi.bi_compression != 0
    {
        panic!("Unsupported file format");
    }

    let height = bi.bi_height.abs();
    let width = bi.bi_width;
    let byte_alignment = 4;
    let rgb_width = 3;;

    // Determine padding for scanlines
    let padding = (byte_alignment - ((width * rgb_width) % byte_alignment)) % byte_alignment;
    let mut rgb_triples: Vec<Vec<RGBTriple>> = Vec::with_capacity((height) as usize);

    for _ in 0..height {
        let mut rgb_triple_row: Vec<RGBTriple> = Vec::with_capacity((width) as usize);
        for _ in 0..width {
            let mut byte_triple: [u8; 3] = [0; 3];
            file.read_exact(&mut byte_triple)?;
            let rgb_triple = RGBTriple::from_u8_bytes(&byte_triple);
            rgb_triple_row.push(rgb_triple);
        }
        // Skip over padding, if any
        file.seek(SeekFrom::Current(padding as i64))?;
        rgb_triples.push(rgb_triple_row);
    }

    Ok((bf, bi, rgb_triples))
}


pub fn write_bmp(filename: &str, bf: &BITMAPFILEHEADER, bi: &BITMAPINFOHEADER, rgb_triple: &Vec<Vec<RGBTriple>>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let height = bi.bi_height.abs();
    let width = bi.bi_width;
    let byte_alignment = 4;
    let rgb_width = 3;
    // Determine padding for scanlines
    let padding = (byte_alignment - ((width * rgb_width) % byte_alignment)) % byte_alignment;
    write_out_struct(bf, &mut file)?;
    write_out_struct(bi, &mut file)?;

    for row in rgb_triple {
        for item in row {
            file.write_all(&item.to_u8_bytes())?;
        }
        file.write_all(&vec![0; padding as usize].as_slice())?;
    }
    Ok(())
}