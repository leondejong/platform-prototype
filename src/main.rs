use platform_prototype::display::run;

use platform_prototype::game::data::{FILTER, HEIGHT, RESIZABLE, SCALE, TITLE, WIDTH};
use platform_prototype::game::state::State;

fn main() {
    let state = State::build();
    run(WIDTH, HEIGHT, SCALE, RESIZABLE, FILTER, TITLE.into(), state);
}
