use winit::keyboard::KeyCode;

use crate::world::World;

use super::{State, PARTICLE_RADIUS};

pub fn update(scale: f32, state: &mut State<'_>, world: &mut World) {
    let radius = PARTICLE_RADIUS * scale;

    if let Some(dragging) = state.dragging {
        world.particles[dragging].0 = state.mouse;

        if !state.mouse_down[0] {
            state.dragging = None;
        }

        return;
    }

    let mut i = 0;
    while i < world.particles.len() {
        let (pos, _) = &mut world.particles[i];

        let hovered = (*pos - state.mouse).magnitude() < radius;
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

pub fn on_key(scale: f32, state: &mut State, world: &mut World, key: KeyCode) {
    let radius = PARTICLE_RADIUS * scale;

    let hovering = world
        .particles
        .iter_mut()
        .enumerate()
        .find(|(_, (pos, _))| (*pos - state.mouse).magnitude() < radius);

    match key {
        KeyCode::Equal => {
            if let Some((_, (_, charge))) = hovering {
                *charge += 1;
            } else {
                world.particles.push((state.mouse, 1));
            }
        }
        KeyCode::Minus => {
            if let Some((_, (_, charge))) = hovering {
                *charge -= 1;
            } else {
                world.particles.push((state.mouse, -1));
            }
        }
        _ => {}
    }

    world.particles.retain(|(_, charge)| *charge != 0);
}
