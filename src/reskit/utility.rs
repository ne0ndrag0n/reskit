pub fn print_error( error_msg: &str ) {
	red!( "fatal: " ); println!( "{}", error_msg );
}

pub fn print_good( message: &str ) {
	green!( "success: " ); println!( "{}", message );
}