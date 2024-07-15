use platform_prototype::display::run;

use platform_prototype::game::data::{HEIGHT, RESIZABLE, SCALE, TITLE, WIDTH};
use platform_prototype::game::state::State;

fn main() {
    let state = State::new();
    run(WIDTH, HEIGHT, SCALE, RESIZABLE, TITLE.into(), state);
}
