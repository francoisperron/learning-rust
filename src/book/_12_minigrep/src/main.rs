use _12_minigrep::config::Config;
use _12_minigrep::minigrep;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from(env::args())?;
    minigrep(config)?;
    Ok(())
}
