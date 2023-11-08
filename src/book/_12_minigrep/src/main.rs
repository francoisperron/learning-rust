use std::env;
use _12_minigrep::minigrep;
use _12_minigrep::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(env::args())?;
    minigrep(config)?;
    Ok(())
}