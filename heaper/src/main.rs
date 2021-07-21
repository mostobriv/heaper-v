extern crate nix;

extern crate clap;

use clap::{Arg, App, SubCommand};

mod web;
mod executor;

use web::WebServer;


fn main() {
    let matches = App::new("Heaper-V")
                    .version("1.0")
                    .arg(Arg::with_name("binary")
                        .value_name("BINARY_PATH")
                        .required(true)
                        .takes_value(true)
                        .help("Path to binary to trace"))
                    .arg(Arg::with_name("breakpoint")
                        .value_name("ADDRESS1 ADDRESS2 ...")
                        .short("b")
                        .long("break")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                        .min_values(1)
                        .help("Breakpoint address[es]"))
                    .arg(Arg::with_name("main_arena_offset")
                        .value_name("OFFSET")
                        .short("m")
                        .long("main_arena_offset")
                        .required(true)
                        .takes_value(true)
                        .help("Relative offset to MAIN_ARENA symbol in libc"))
                .get_matches_from(vec!["test", "test_binary", "-m", "0x7f005000", "--break", "1", "2", "3", "4", "-b", "0x1337"]);
    

    let breakpoints: Vec<usize> = matches.values_of("breakpoint")
                                        .expect("Error due getting a list of breakpoints")
                                        .map(|x| {
                                            if let Some(num_str) = x.strip_prefix("0x") {
                                                usize::from_str_radix(num_str, 16).unwrap()
                                            } else {
                                                if let Ok(num_str) = x.parse() { num_str } else { usize::from_str_radix(x, 16).unwrap() }
                                            }
                                        }).collect();
    
    let server = WebServer::new("127.0.0.1", 31337);
    
}
