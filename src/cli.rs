use bevy::prelude::*;
use clap::Parser;
use std::path::PathBuf;

/// CLI Arguments for the Helix 3D Renderer
#[derive(Parser, Resource, Debug, Clone)]
#[command(name = "helix-3d-renderer")]
#[command(about = "3D Renderer for Dota 2 hero models with animation playback")]
#[command(version)]
pub struct CliArgs {
    /// Screenshot mode: capture frame and exit
    #[arg(
        long,
        value_name = "PATH",
        help = "Capture screenshot to file and exit"
    )]
    pub screenshot: Option<PathBuf>,

    /// Render to image without window
    #[arg(long, value_name = "PATH", help = "Headless render to image file")]
    pub render: Option<PathBuf>,

    /// Frame to capture (for animation screenshots)
    #[arg(
        long,
        default_value = "60",
        help = "Wait N frames before capturing screenshot"
    )]
    pub capture_frame: u32,

    /// Animation to play (index 0-9)
    #[arg(
        long,
        default_value = "0",
        help = "Animation index to play (0=Idle, 1=Run, etc.)"
    )]
    pub animation: usize,

    /// Output directory for automated testing
    #[arg(
        long,
        value_name = "DIR",
        help = "Directory for automated test outputs"
    )]
    pub test_output: Option<PathBuf>,

    /// Run in headless mode (no window)
    #[arg(long, help = "Run in headless mode without window")]
    pub headless: bool,

    /// Verbose logging
    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,

    /// Accessory configuration (comma-separated list of names to disable)
    #[arg(
        long,
        value_name = "ACCESSORIES",
        help = "Disable specific accessories (comma-separated)"
    )]
    pub disable_accessories: Option<String>,

    /// Remote logging endpoint (requires remote-log feature)
    #[arg(
        long,
        value_name = "URL",
        help = "Remote log endpoint URL (requires --features remote-log)"
    )]
    pub log_endpoint: Option<String>,
}
