use filter_more::bmp::{read_bmp, write_bmp};
use filter_more::cli::{CommandOptions, CliFlag};
use filter_more::filter;

fn main() {
    match CommandOptions::build() {
        Ok(opts) => {
            let (bmp_header, bmp_info, mut rgb_triple) = read_bmp(&opts.source).unwrap();
            match opts.flag {
                CliFlag::Blur => { filter::blur(&mut rgb_triple); }
                CliFlag::Edges => { filter::edges(&mut rgb_triple) }
                CliFlag::GreyScale => { filter::grey_scale(&mut rgb_triple); }
                CliFlag::Reflect => { filter::reflect(&mut rgb_triple) }
            };
            write_bmp(&opts.output, &bmp_header, &bmp_info, &rgb_triple).unwrap();
        },
        Err(message) => {
            println!("{message}");
            return
        }
    };
}