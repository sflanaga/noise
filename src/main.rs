mod noise;

#[macroquad::main("Noise Visualization")]
async fn main() {
    // Directly run the noise simulation for the browser version
    noise::run().await;
}
