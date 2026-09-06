//! Scripts for Helix Golf
use std::env;

use miette::miette;

mod command;
mod generate_helix_config;
mod generate_tape_file;
mod parse_example;
mod parse_helix_keys;
use command::Command;

fn main() -> miette::Result<()> {
    env::args()
        .nth(1)
        .ok_or(Command::ERROR)
        .and_then(|arg| arg.parse::<Command>())
        .map_err(|err| miette!("{err}"))?
        .execute()
}
