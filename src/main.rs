mod flux;

use anyhow::Result;
use glam::uvec2;
use measure_time::trace_time;

use crate::flux::render_image;

fn main() -> Result<()> {
    env_logger::init();
    trace_time!("main");

    let resolution = uvec2(800, 600);
    let img = render_image(resolution);
    img.save("./output/output.png")?;

    Ok(())
}
