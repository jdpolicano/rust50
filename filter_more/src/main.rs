mod cli;
mod bmp;

use std::env::args;
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

    let (bm_header, bm_info, mut rgb_triple) = read_bmp(&options.source).unwrap();

    match options.flag {
        CliFlag::Blur => { println!("todo!"); }
        CliFlag::Edges => { println!("todo!"); }
        CliFlag::GreyScale => { println!("todo!"); }
        CliFlag::Reflect => { println!("todo!"); }
    };

    let _ = write_bmp(&options.output, &bm_header, &bm_info, &rgb_triple).unwrap();
}
