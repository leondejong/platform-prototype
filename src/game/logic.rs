use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;

use crate::canvas;

use super::data::Sprite;
use super::state::State;

// Update state
pub fn update_state(state: &mut State, time: f32, delta: f32, fps: f32) {
    state.conf.time = time;
    state.conf.delta = delta;
    state.conf.fps = fps;

    if state.conf.delta > state.conf.max {
        state.conf.delta = state.conf.max;
    };

    state.conf.accumulator += state.conf.delta;

    update_direction(state);

    // Timestepping
    while state.conf.accumulator > state.conf.step {
        state.conf.accumulator -= state.conf.step;
        compute_physics(state); // Linear integration
    }

    // Linear interpolation
    let alpha = state.conf.accumulator / state.conf.step;
    state.sub.interpolation = interpolate_coordinates(alpha, state);
}

// Render graphics
pub fn render_graphics(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    let message = |id: u32| format!("Texture with id '{}' should exist", id);

    let x = state.sub.interpolation.x - state.env.x;
    let y = state.sub.interpolation.y - state.env.y;

    state.env.background.set_x_y(-state.env.x, -state.env.y);

    canvas::image::render(buffer, width, height, &state.env.background);

    for tile in state.env.tiles.iter() {
        let x = tile.position.x - state.env.x;
        let y = tile.position.y - state.env.y;

        if exceeds_viewport(&tile.boundary, &state.view()) {
            continue;
        }

        let image = state
            .env
            .textures
            .get_mut(&tile.id)
            .expect(&message(tile.id));

        image.set_x(x);
        image.set_y(y);

        canvas::image::render(buffer, width, height, image);
    }

    let frame_x = x + state.sub.offset.x;
    let frame_y = y + state.sub.offset.y;

    let frame = state.sub.animations.consecutive_frame(frame_x, frame_y);

    canvas::image::render(buffer, width, height, frame);
}

// Calculate and update physics
pub fn compute_physics(state: &mut State) {
    let ax = state.sub.force.x / state.sub.mass;
    let ay = state.sub.force.y / state.sub.mass;

    let ix = state.sub.impulse.x * state.sub.direction.x;
    let iy = state.sub.impulse.y * state.sub.direction.y;

    state.sub.acceleration.x = (ax + ix) * state.conf.step;
    state.sub.acceleration.y = (ay + iy + state.env.gravity) * state.conf.step;

    state.sub.velocity.x += state.sub.acceleration.x * state.conf.ratio;
    state.sub.velocity.y += state.sub.acceleration.y * state.conf.ratio;

    let [dx, dy, cx, cy] = collision_delta(state);

    state.sub.velocity.x = if cx.abs() < 1.0 { dx } else { 0.0 };
    state.sub.velocity.y = if cy.abs() < 1.0 { dy } else { 0.0 };

    state.sub.velocity.x *= state.env.dissipation.x;
    state.sub.velocity.y *= state.env.dissipation.y;

    if cx != 0.0 {
        state.sub.velocity.y *= state.env.friction;
    } else if cy != 0.0 {
        state.sub.velocity.x *= state.env.friction;
    } else {
        state.sub.velocity.x *= state.env.resistance;
        state.sub.velocity.y *= state.env.resistance;
    }

    state.sub.x += dx;
    state.sub.y += dy;

    state.env.x = state.sub.x + state.sub.width / 2.0 - state.conf.width / 2.0;
    state.env.y = state.sub.y + state.sub.height / 2.0 - state.conf.height / 2.0;

    jump_player(cy, state);

    update_animation(state, dx, dy);

    constrain_map(state);
}

// Generate and render background pattern
pub fn generate_background(state: &State, buffer: &mut [u8], width: u32, height: u32) {
    let mut pattern = state.env.pattern.clone();

    let sx = state.env.x as i32;
    let sy = state.env.y as i32;

    let sw = state.conf.width as i32;
    let sh = state.conf.height as i32;

    let w = pattern.width() as i32;
    let h = pattern.height() as i32;

    let tx = sx % w;
    let ty = sy % h;

    let mut x = 0;
    let mut y = 0;

    while x < sw + w {
        while y < sh + h {
            pattern.set_x_y((x - tx) as f32, (y - ty) as f32);
            canvas::image::render(buffer, width, height, &pattern);
            y += h;
        }
        x += w;
        y = 0;
    }
}

// Check if objects intersect
pub fn detect_intersection(a: &Rectangle, b: &Rectangle) -> bool {
    return a.x < b.x + b.width
        && b.x < a.x + a.width
        && a.y < b.y + b.height
        && b.y < a.y + a.height;
}

// Get collision delta
pub fn collision_delta(state: &State) -> [f32; 4] {
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut dx = state.sub.velocity.x;
    let mut dy = state.sub.velocity.y;

    let subject = &Rectangle::new(state.sub.x, state.sub.y, state.sub.width, state.sub.height);

    let horizontal = &Rectangle::new(
        state.sub.x + dx,
        state.sub.y,
        state.sub.width,
        state.sub.height,
    );

    let vertical = &Rectangle::new(
        state.sub.x,
        state.sub.y + dy,
        state.sub.width,
        state.sub.height,
    );

    for tile in state.env.tiles.iter() {
        let rectangle = tile.boundary;

        if exceeds_viewport(&rectangle, &state.view()) {
            continue;
        }

        if detect_intersection(&rectangle, horizontal) {
            if dx < 0.0 {
                dx = rectangle.x + rectangle.width - subject.x;
            } else if dx > 0.0 {
                dx = rectangle.x - subject.x - subject.width;
            }
            cx = state.sub.velocity.x - dx;
        }

        if detect_intersection(&rectangle, vertical) {
            if dy < 0.0 {
                dy = rectangle.y + rectangle.height - subject.y;
            } else if dy > 0.0 {
                dy = rectangle.y - subject.y - subject.height;
            }
            cy = state.sub.velocity.y - dy;
        }
    }

    // Delta x and y, correction x and y
    [dx, dy, cx, cy]
}

// Interpolate subject position
pub fn interpolate_coordinates(alpha: f32, state: &mut State) -> Point {
    let x = state.sub.previous.x * alpha + state.sub.x * (1.0 - alpha);
    let y = state.sub.previous.y * alpha + state.sub.y * (1.0 - alpha);

    state.sub.previous.x = state.sub.x;
    state.sub.previous.y = state.sub.y;

    Point { x, y }
}

// Constrain level to viewport
pub fn constrain_map(state: &mut State) {
    if state.env.x < 0.0 {
        state.env.x = 0.0;
    }

    if state.env.x > state.env.width - state.conf.width {
        state.env.x = state.env.width - state.conf.width;
    }

    if state.env.y < 0.0 {
        state.env.y = 0.0;
    }

    if state.env.y > state.env.height - state.conf.height {
        state.env.y = state.env.height - state.conf.height;
    }
}

// Check if object clips viewport
pub fn exceeds_viewport(object: &Rectangle, view: &Rectangle) -> bool {
    if object.x as i32 > (view.x + view.width) as i32 {
        return true;
    }

    if view.x as i32 > (object.x + object.width) as i32 {
        return true;
    }

    if object.y as i32 > (view.y + view.height) as i32 {
        return true;
    }

    if view.y as i32 > (object.y + view.height) as i32 {
        return true;
    }

    return false;
}

// Update subject direction
pub fn update_direction(state: &mut State) {
    state.sub.direction.x = 0.0;
    state.sub.direction.y = 0.0;

    if state.conf.left && !state.conf.right {
        state.sub.direction.x = -1.0;
    }

    if state.conf.right && !state.conf.left {
        state.sub.direction.x = 1.0;
    }

    if state.conf.up && !state.conf.down {
        state.sub.direction.y = -1.0;
    }

    if state.conf.down && !state.conf.up {
        state.sub.direction.y = 1.0;
    }
}

// Player jump logic
pub fn jump_player(overlap: f32, state: &mut State) {
    state.sub.contact = false;

    if state.conf.jump && !state.sub.lock && overlap > 0.0 {
        state.sub.lock = true;
        state.sub.contact = true;
    }

    if !state.conf.jump && state.sub.lock && overlap > 0.0 {
        state.sub.lock = false;
    }

    if state.sub.contact {
        state.sub.velocity.y -= state.sub.jump * state.conf.ratio;
    }
}

// Set player animation
pub fn update_animation(state: &mut State, dx: f32, dy: f32) {
    let kr = state.conf.right; // Key right
    let kl = state.conf.left; // Key left
    let ku = state.conf.up; // Key up
    let kd = state.conf.down; // Key down
    let mx = 0.1; // Margin x
    let my = 0.1; // Margin y
    let ir = dy.abs() > my; // In air

    if dx < -mx && kl && !ir {
        state.sub.animations.set(&Sprite::RunningLeft.str()); // Running left
    } else if dx > mx && kr && !ir {
        state.sub.animations.set(&Sprite::RunningRight.str()); // Running right
    } else if dy < -my && ku && !ir {
        state.sub.animations.set(&Sprite::RunningUp.str()); // Running up
    } else if dy > my && kd && !ir {
        state.sub.animations.set(&Sprite::RunningDown.str()); // Running down
    } else if ir && kl {
        state.sub.animations.set(&Sprite::AirLeft.str()); // In air, moving left
    } else if ir && kr {
        state.sub.animations.set(&Sprite::AirRight.str()); // In air, moving right
    } else if ir && ku {
        state.sub.animations.set(&Sprite::AirUp.str()); // In air, moving up
    } else if ir && kd {
        state.sub.animations.set(&Sprite::AirDown.str()); // In air, moving down
    } else if dx < -mx {
        state.sub.animations.set(&Sprite::FacingLeft.str()); // Facing left
    } else if dx > mx {
        state.sub.animations.set(&Sprite::FacingRight.str()); // Facing right
    } else if dy < -my {
        state.sub.animations.set(&Sprite::FacingUp.str()); // Facing up
    } else if dy > my {
        state.sub.animations.set(&Sprite::FacingDown.str()); // Facing down
    } else if ir {
        state.sub.animations.set(&Sprite::AirDown.str()); // In air
    } else {
        state.sub.animations.set(&Sprite::FacingDown.str()); // Inactive
    }
}
