extern crate spawn_ptrace;

use spawn_ptrace::CommandPtraceSpawn;
use std::process::Command;


struct Breakpoint {
    address: usize,
}


struct Executor {
    breakpoints: Vec<Breakpoint>,
    binary_path: String,
}


impl Executor {
    pub fn new (offsets: Option<Vec<usize>>, binary_path: &str) -> Self {
        Executor {
            breakpoints: if let Some(offsets) = offsets { offsets.into_iter().map(|x| Breakpoint { address: x }).collect() } else { Vec::new() },
            binary_path: String::from(binary_path)
        }   
    }

    pub fn run() {

    }
}