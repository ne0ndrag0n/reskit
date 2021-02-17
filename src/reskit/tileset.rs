use crate::reskit::utility;
use std::process::exit;
use std::fs::File;
use std::io::Write;
use image::{ GenericImageView, DynamicImage };

fn color_to_palette( r: u16, g: u16, b: u16, palette: &mut [u16; 16] ) -> u32 {
	let final_val =
		( ( r & 0x00F0 ) >> 4 ) |
		( g & 0x00F0 ) |
		( ( b & 0x00F0 ) << 4 );

	// Does the color already exist?
	for i in 0..palette.len() {
		if palette[ i ] == final_val {
			return i as u32;
		}
	}

	// Place the colour in the next open slot
	for i in 1..palette.len() {
		if palette[ i ] == 0 {
			palette[ i ] = final_val;
			return i as u32;
		}
	}

	utility::print_error( "image contains greater than 15 colours, exiting..." );
	exit( 3 );
}

fn get_pixel( image: &DynamicImage, palette: &mut [u16; 16], x: u32, y: u32 ) -> u32 {
	let ( max_x, max_y ) = image.dimensions();

	if x >= max_x || y >= max_y {
		return 0;
	}

	let pixel = image.get_pixel( x, y );
	color_to_palette( pixel[ 0 ].into(), pixel[ 1 ].into(), pixel[ 2 ].into(), palette )
}

pub fn generate( image_filename: &str, output_filename: &str ) {
	let img = image::open( image_filename );
	if let Ok( img ) = img {
		let ( mut max_x, mut max_y ) = img.dimensions();
		if max_x % 8 != 0 { max_x = ( 8 * ( max_x / 8 ) ) + ( 8 - ( max_x % 8 ) ); }
		if max_y % 8 != 0 {	max_y = ( 8 * ( max_y / 8 ) ) + ( 8 - ( max_y % 8 ) ); }

		let mut palette: [u16; 16] = [ 0; 16 ];
		let mut body: Vec< u8 > = Vec::new();

		for y in ( 0..max_y ).step_by( 8 ) {
            for x in ( 0..max_x ).step_by( 8 ) {
				for cell_y in 0..8 {
					let mut series: u32 = 0;

					for cell_x in 0..8 {
						let nibble: u32 = get_pixel( &img, &mut palette, cell_x + x, cell_y + y ) << ( ( 7 - cell_x ) * 4 );
						series = series | nibble;
					}

					let bytes = series.to_be_bytes();
					for i in 0..4 {
						body.push( bytes[ i ] );
					}
				}
			}
		}

		let mut output_palette: Vec< u8 > = Vec::new();
		for i in 0..palette.len() {
			let bytes = palette[ i ].to_be_bytes();
			for i in 0..2 {
				output_palette.push( bytes[ i ] );
			}
		}

		let output_try = File::create( output_filename );
		if let Ok( mut output_file ) = output_try {
			output_file.write( &output_palette ).unwrap();
			output_file.write( &body ).unwrap();
			utility::print_good( format!( "converted file {}", image_filename ).as_str() );
		} else {
			utility::print_error( format!( "could not open filename for output {}", output_filename ).as_str() );
		}
	} else {
		utility::print_error( format!( "could not open filename {}", image_filename ).as_str() );
	}
}