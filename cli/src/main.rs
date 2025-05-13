mod register;

use clap::Parser;
use log::{info, warn};
use interfaces::logger;
use interfaces::models::Settings;
use crate::register::Register;

#[derive(Parser, Debug)]
enum Cli {
    Register(Register)
}

fn main() -> anyhow::Result<()> {
    let sets = Settings::load();
    logger::bind_logger(&sets)?;
    match Cli::parse() {
        Cli::Register(regs) => info!("add"),
    };
    sets.save()?;
    Ok(())
}
