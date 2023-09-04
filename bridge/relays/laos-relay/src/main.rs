//! LAOS bridge relay entrypoint.

#![warn(missing_docs)]

mod bridges;
mod chains;
mod cli;

fn main() {
    let command = cli::parse_args();
    let run = command.run();
    async_std::task::block_on(run);
}
