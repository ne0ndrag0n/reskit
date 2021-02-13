pub fn print_error( error_msg: &'static str ) {
	red!( "fatal: " ); println!( "{}", error_msg );
}