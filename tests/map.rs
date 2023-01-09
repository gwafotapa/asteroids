use asteroids::{camera, map, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[test]
fn count_stars() {
    let mut app = App::new();
    app.add_plugins(TestPlugins)
        .add_event::<map::star::StarsEvent>()
        .add_startup_system(camera::spawn)
        .add_startup_system(map::spawn)
        .add_system(map::update)
        .add_system(map::star::spawn.after(map::update));

    app.update();

    assert_eq!(
        app.world
            .query::<&map::star::Star>()
            .iter(&app.world)
            .count(),
        9 * map::star::STARS_PER_SECTOR
    );

    // Move camera to sector (1, 1)
    let (mut c_transform, _) = app
        .world
        .query::<(&mut Transform, With<Camera>)>()
        .single_mut(&mut app.world);
    c_transform.translation.x = 1.5 * WINDOW_WIDTH;
    c_transform.translation.y = 1.5 * WINDOW_HEIGHT;

    app.update();

    assert_eq!(
        app.world
            .query::<&map::star::Star>()
            .iter(&app.world)
            .count(),
        9 * map::star::STARS_PER_SECTOR
    );

    // Move camera to sector (2, 1)
    let (mut c_transform, _) = app
        .world
        .query::<(&mut Transform, With<Camera>)>()
        .single_mut(&mut app.world);
    c_transform.translation.x = 2.5 * WINDOW_WIDTH;
    c_transform.translation.y = 1.5 * WINDOW_HEIGHT;

    app.update();

    assert_eq!(
        app.world
            .query::<&map::star::Star>()
            .iter(&app.world)
            .count(),
        9 * map::star::STARS_PER_SECTOR
    );

    // Move camera back to sector (1, 1)
    let (mut c_transform, _) = app
        .world
        .query::<(&mut Transform, With<Camera>)>()
        .single_mut(&mut app.world);
    c_transform.translation.x = 1.5 * WINDOW_WIDTH;
    c_transform.translation.y = 1.5 * WINDOW_HEIGHT;

    app.update();

    assert_eq!(
        app.world
            .query::<&map::star::Star>()
            .iter(&app.world)
            .count(),
        9 * map::star::STARS_PER_SECTOR
    );
}

struct TestPlugins;

impl PluginGroup for TestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(bevy::log::LogPlugin::default())
            .add(bevy::core::CorePlugin::default())
            .add(bevy::time::TimePlugin::default())
            // .add(bevy::transform::TransformPlugin::default())
            // .add(bevy::hierarchy::HierarchyPlugin::default())
            // .add(bevy::diagnostic::DiagnosticsPlugin::default())
            // .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::asset::AssetPlugin::default())
            // .add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default())
            // .add(bevy::scene::ScenePlugin::default())
            // .add(bevy::winit::WinitPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::render::texture::ImagePlugin::default())
            .add(bevy::core_pipeline::CorePipelinePlugin::default())
            .add(bevy::sprite::SpritePlugin::default())
        // .add(bevy::text::TextPlugin::default())
        // .add(bevy::ui::UiPlugin::default())
        // .add(bevy::pbr::PbrPlugin::default())
        // .add(bevy::gltf::GltfPlugin::default())
        // .add(bevy::audio::AudioPlugin::default())
        // .add(bevy::gilrs::GilrsPlugin::default())
        // .add(bevy::animation::AnimationPlugin::default())
    }
}
