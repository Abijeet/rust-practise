extern crate clap;

pub mod dom;
pub mod html;

use clap::{Arg, App};

fn main() {
    App::new("simple browser")
        .version("1.0")
        .about("A simple toy web browser!")
        .author("Abijeet Patro")
        .arg(Arg::with_name("html")
            .short("t")
            .long("html")
            .value_name("FILENAME")
            .help("The HTML file to be parsed")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("css")
            .short("c")
            .long("css")
            .value_name("FILENAME")
            .help("The corresponding CSS file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILENAME")
            .help("The output file")
            .takes_value(true))
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .value_name("png | pdf")
            .help("The output format - png / pdf.")
            .takes_value(true)
            .possible_values(&[
                "pdf",
                "png"
            ])
            .default_value("png"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Verbosity level"))
        .get_matches();
}
