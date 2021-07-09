extern crate nix;
extern crate clap;
extern crate spawn_ptrace;

use spawn_ptrace::CommandPtraceSpawn;
use std::process::Command;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Heaper-V")
                    .version("1.0")
                    .arg(Arg::with_name("binary")
                        .value_name("BINARY_PATH")
                        .required(true)
                        .takes_value(true)
                        .help("Path to binary to trace"))
                    .arg(Arg::with_name("breakpoint")
                        .value_name("ADDRESS1, ADDRESS2, ...")
                        .short("b")
                        .long("break")
                        .required(true)
                        .takes_value(true)
                        .min_values(1)
                        .help("Breakpoint address[es]"))
                    .arg(Arg::with_name("main_arena_offset")
                        .value_name("OFFSET")
                        .short("m")
                        .long("main_arena_offset")
                        .required(true)
                        .takes_value(true)
                        .help("Relative offset to MAIN_ARENA symbol in libc"))
                .get_matches_from(vec!["test", "--help"]);
}
