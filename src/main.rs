mod noise;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use noise::NoiseMode;
use noise::Config;

#[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    let config = {
        let cli = Cli::parse();
        Config {
            mode: cli.mode,
            bw: cli.bw,
            fps: cli.fps,
            lines: cli.lines,
        }
    };
    #[cfg(target_arch = "wasm32")]
    let config = Config::default();
    noise::run(config).await;
}
