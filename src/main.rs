extern crate image;
#[macro_use]
extern crate colour;

use std::env::{ Args, args };
use std::process::exit;

mod reskit;
use reskit::tileset;

struct ArgumentOption {
    pub key: String,
    pub value: String
}

fn print_help() {
    println!( "Usage:" );
    println!( "reskit [plugin] options" );
    println!( "" );
    println!( "Available Plugins:" );
    println!( "   tileset - Convert 15-colour PNG to Sega Genesis VDP format" );
    println!( "             Output as .bin containing palette, then tiles");
    println!( "" );
    println!( "Options:" );
    println!( "--input         Specify input file for plugin" );
    println!( "--output        Specify output file/path for plugin" );
}

fn get_option( arguments: &mut Args ) -> Option< ArgumentOption > {
    Some( ArgumentOption{ key: arguments.next()?, value: arguments.next()? } )
}

fn load_module( module: String, arguments: Args ) -> i32 {
    // Discriminate module
    match module.as_str() {
        "reskit" => {
            42
        },
        _ => {
            red!( "fatal: " ); println!( "invalid module: {}", module );
            2
        }
    }
}

fn main() {
    cyan!( "reskit" ); println!( " - Sega Genesis Resource Kit (v0.0.1a)" );
    println!( "(c) 2021 Ashley N. (ne0ndrag0n)" );

    let mut args = args();
    args.next(); // Burn arg1

    let mode = match args.next() {
        Some( mode ) => mode,
        None => {
            print_help();
            exit( 1 );
        }
    };

    exit( load_module( mode, args ) );
}
