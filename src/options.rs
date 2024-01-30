use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    pub img1: PathBuf,
    pub img2: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
