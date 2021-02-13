use crate::reskit::utility;
use image::GenericImageView;

pub fn generate( image_filename: &str, output_filename: &str ) {
	let img = image::open( image_filename );
	if let Ok( img ) = img {
		// Image must be multiple of 8 in both dimensions
		let x = img.dimensions().0;
		let y = img.dimensions().1;

	} else {

	}
}