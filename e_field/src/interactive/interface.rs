use winit::keyboard::KeyCode;

use crate::world::World;

use super::{State, PARTICLE_RADIUS};

pub fn update(state: &mut State<'_>, world: &mut World) {
    if let Some(dragging) = state.dragging {
        world.particles[dragging].0 += state.mouse_delta;

        if !state.mouse_down[0] {
            state.dragging = None;
        }

        return;
    }

    let mut i = 0;
    while i < world.particles.len() {
        let (pos, _) = &mut world.particles[i];

        let hovered = (*pos - state.last_mouse).magnitude() < PARTICLE_RADIUS;
        if hovered {
            if state.mouse_down[0] {
                state.dragging = Some(i);
            } else if state.mouse_down[1] {
                world.particles.remove(i);
            }
            break;
        }

        i += 1;
    }
}

pub fn on_key(state: &mut State, world: &mut World, key: KeyCode) {
    let hovering = world
        .particles
        .iter_mut()
        .find(|(pos, _)| (*pos - state.last_mouse).magnitude() < PARTICLE_RADIUS);

    match key {
        KeyCode::Equal => {
            if let Some((_, charge)) = hovering {
                *charge += 1;
            } else {
                world.particles.push((state.last_mouse, 1));
            }
        }
        KeyCode::Minus => {
            if let Some((_, charge)) = hovering {
                *charge -= 1;
            } else {
                world.particles.push((state.last_mouse, -1));
            }
        }
        _ => {}
    }
}
