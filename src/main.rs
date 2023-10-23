mod flux;

use glam::uvec2;

fn main() -> anyhow::Result<()> {
    let resolution = uvec2(800, 600);
    let img = flux::render_image(resolution);
    img.save("./output/output.png")?;

    Ok(())
}
