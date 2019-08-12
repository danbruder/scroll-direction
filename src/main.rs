use quicli::prelude::*;
use std::process::Command;
use std::str::FromStr;
use std::string::ToString;
use structopt::StructOpt;

// Add cool slogan for your app here, e.g.:
/// Get and set the mouse scroll direction
#[derive(Debug, StructOpt)]
#[structopt(name = "scroll", about = "get and set the scroll direction")]
enum Cli {
    #[structopt(name = "get")]
    Get {},
    #[structopt(name = "set")]
    Set { value: Direction },
}

#[derive(Debug)]
enum Direction {
    Natural,
    NotNatural,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Natural => write!(f, "natural"),
            Direction::NotNatural => write!(f, "not natural"),
        }
    }
}

#[derive(Debug)]
struct AppError(String);

impl FromStr for Direction {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Direction::NotNatural),
            "1" => Ok(Direction::Natural),
            _ => Err(AppError("Invalid direction".to_owned())),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError(format!("{:?}", err).to_owned())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_owned())
    }
}
impl std::error::Error for AppError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn get() -> Result<Direction, AppError> {
    let output = Command::new("defaults")
        .arg("read")
        .arg("-g")
        .arg("com.apple.swipescrolldirection")
        .output()?;
    if output.status.code() == Some(0) {
        let value = String::from_utf8_lossy(&output.stdout);
        let value = value.trim();
        if value == "0".to_owned() {
            return Ok(Direction::NotNatural);
        } else if value == "1".to_owned() {
            return Ok(Direction::Natural);
        }
    }

    Err(AppError("Could not read scroll direction".to_owned()))
}

fn set(direction: Direction) -> Result<(), AppError> {
    let val = match direction {
        Direction::Natural => "TRUE",
        Direction::NotNatural => "FALSE",
    };

    let output = Command::new("defaults")
        .arg("write")
        .arg("-g")
        .arg("com.apple.swipescrolldirection")
        .arg("-bool")
        .arg(val)
        .output()?;

    if output.status.code() == Some(0) {
        return Ok(());
    }

    Err(AppError("Could not read scroll direction".to_owned()))
}

fn main() -> CliResult {
    let args = Cli::from_args();

    match args {
        Cli::Get {} => {
            let direction = get()?;

            println!("Scroll direction is currently {}", direction);
        }
        Cli::Set { value } => {
            println!("Set to {}", value);
            set(value)?;
        }
    }

    Ok(())
}
