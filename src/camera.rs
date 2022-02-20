use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CameraFollowing;

pub struct CameraPlugin;

const DEFAULT_PROJECTION: f32 = 1. / 4.;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(toggle_projection)
            .add_system_to_stage(CoreStage::Last, camera_follow_player);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // Set the scale on the window right away
    // let main_window = windows.get_primary().unwrap();
    // camera.orthographic_projection.scale = config::WINDOW_WIDTH / main_window.width();
    camera.orthographic_projection.scale = DEFAULT_PROJECTION;

    // commands.spawn_bundle(camera).insert(ScalingCamera);
    commands.spawn_bundle(camera);
    // commands.spawn_bundle(UiCameraBundle::default());
}

fn toggle_projection(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut OrthographicProjection>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        for mut projection in query.iter_mut() {
            projection.scale = if projection.scale == 1. {
                DEFAULT_PROJECTION
            } else {
                1.
            }
        }
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, (With<CameraFollowing>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}
