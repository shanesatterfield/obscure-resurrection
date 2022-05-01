use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CameraFollowing;

#[derive(Component, Default)]
pub struct CameraThatFollows;

pub struct CameraPlugin;

pub const WINDOW_SCALE: f32 = 4.;

pub const DEFAULT_PROJECTION: f32 = 1. / WINDOW_SCALE;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(toggle_projection)
            .add_system_to_stage(CoreStage::Update, camera_follow_player);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // Set the scale on the window right away
    camera.orthographic_projection.scale = DEFAULT_PROJECTION;

    commands
        .spawn_bundle(camera)
        .insert(CameraThatFollows::default());

    commands.spawn_bundle(UiCameraBundle::default());
}

fn toggle_projection(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut OrthographicProjection>,
) {
    if keyboard_input.just_released(KeyCode::M) {
        for mut projection in query.iter_mut() {
            projection.scale = if projection.scale == 1. {
                DEFAULT_PROJECTION
            } else {
                1.
            }
        }
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<CameraFollowing>, Without<CameraThatFollows>)>,
    mut camera_query: Query<&mut Transform, With<CameraThatFollows>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}
