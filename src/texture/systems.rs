use bevy::{prelude::*, render::render_resource::TextureUsages};

use heron::prelude::Velocity;

use super::components::{FacingDirection, HorizontalDirection};

pub fn change_direction(mut query: Query<(&Velocity, &mut FacingDirection), Changed<Velocity>>) {
    for (velocity, mut facing_direction) in query.iter_mut() {
        if velocity.linear.x < 0. {
            facing_direction.0 = HorizontalDirection::LEFT;
        } else if velocity.linear.x > 0. {
            facing_direction.0 = HorizontalDirection::RIGHT;
        }
    }
}

pub fn flip_assets(
    mut query: Query<(&mut TextureAtlasSprite, &FacingDirection), Changed<FacingDirection>>,
) {
    for (mut sprite, facing_direction) in query.iter_mut() {
        match facing_direction.0 {
            HorizontalDirection::LEFT => {
                sprite.flip_x = true;
            }
            HorizontalDirection::RIGHT => {
                sprite.flip_x = false;
            }
        }
    }
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
