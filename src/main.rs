extern crate image;
extern crate clap;
#[macro_use]
extern crate colour;

use clap::{App, SubCommand};

mod reskit;
use reskit::utility;
use reskit::tileset;

fn main() {
    let matches = App::new( "reskit" )
        .version( "0.0.1a" )
        .author( "(c) 2021 Ashley N. <ne0ndrag0n@ne0ndrag0n.com>" )
        .about( "Sega Megadrive resource kit and format converter" )
        .subcommand(
            SubCommand::with_name( "tileset" )
                .about( "Generate a Sega Megadrive VDP tileset + palette from a 15-colour image" )
                .arg_from_usage( "-i, --input=<FILE> 'Specify input image'" )
                .arg_from_usage( "-o, --output=<FILE> 'Specify output file'")
        )
        .get_matches();

    // Get arguments for tileset
    if let Some( matches ) = matches.subcommand_matches( "tileset" ) {
        // Get input and output filenames
        if let Some( input_filename ) = matches.value_of( "input" ) {
            if let Some( output_filename ) = matches.value_of( "output" ) {
                return tileset::generate( input_filename, output_filename );
            } else {
                utility::print_error( "expected: output_filename (-o,--output)" );
            }
        } else {
            utility::print_error( "expected: input filename (-i,--input)" );
        }
    }

    utility::print_error( "no plugin provided (try --help)" );
}
