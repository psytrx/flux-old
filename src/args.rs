use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use log::warn;

use crate::example_scenes::ExampleScene;

pub fn parse_args() -> Result<ParsedArgs> {
    let args = Args::parse();

    if args.dev {
        warn!("dev mode enabled");
    }

    let scene = match ExampleScene::from_str(&args.scene) {
        Ok(scene) => Ok(scene),
        Err(_) => Err(Error::SceneNotFound(args.scene)),
    }?;

    Ok(ParsedArgs {
        dev: args.dev,
        scene,
    })
}

pub struct ParsedArgs {
    pub dev: bool,
    pub scene: ExampleScene,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Runs quick/noisy renders for iterating quickly
    #[arg(short, long)]
    dev: bool,

    /// The example scene to render
    #[arg(short, long)]
    scene: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Scene not found: {0}")]
    SceneNotFound(String),
}
