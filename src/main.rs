mod noise;

use clap::Parser;
use noise::{Config, NoiseMode};

#[derive(Parser)]
#[command(name = "noise", about = "Noise Visualization")]
struct Cli {
    /// Starting noise mode: xorshift, fastrand, hash, pcg, lines, edges, triangles, shaded
    #[arg(short, long, default_value = "pcg")]
    mode: NoiseMode,

    /// Start in black & white mode
    #[arg(short, long)]
    bw: bool,

    /// Show FPS overlay on startup
    #[arg(short, long)]
    fps: bool,

    /// Number of lines per frame in RandomLines mode
    #[arg(short, long, default_value_t = 1000)]
    lines: u32,
}

#[macroquad::main("Noise Visualization")]
async fn main() {
    let cli = Cli::parse();
    let config = Config {
        mode: cli.mode,
        bw: cli.bw,
        fps: cli.fps,
        lines: cli.lines,
    };
    noise::run(config).await;
}
