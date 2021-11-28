use macroquad::prelude::*;

mod biot;
mod biot_collection;

use biot_collection::BiotCollection;

#[macroquad::main("Life")]
async fn main() {

    let mut biots = BiotCollection::new(600);

    loop {
        biots.step();
        clear_background(Color::new(0.,0.,0.1,1.0));
        biots.draw();
        draw_text(&format!("FPS: {}, biots: {}", get_fps(), biots.len()),
            screen_width()-200., screen_height()-5.,
            18.,
            LIGHTGRAY);
        next_frame().await
    }
}
