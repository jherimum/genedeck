use std::io::{Stdin, Stdout};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, stdin: Stdin, stdout: Stdout) {}
}
