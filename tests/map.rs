use asteroids::{camera, map};
use bevy::prelude::*;
use bevy::{
    app::PluginGroupBuilder, prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle,
};
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    A,
    B,
}

pub fn to_b(mut commands: Commands) {
    commands.insert_resource(NextState(State::B));
}

#[test]
fn count_stars() {
    let mut app = App::new();
    app.add_plugins(TestPlugins)
        .add_event::<map::star::StarsEvent>()
        .add_loopless_state(State::A)
        .add_startup_system(camera::spawn)
        .add_system(map::spawn.run_in_state(State::A))
        .add_system(map::star::spawn.run_in_state(State::B))
        .add_system(to_b);
    // .add_system(map::update);

    app.update();

    assert_eq!(
        app.world
            .query::<&map::star::Star>()
            .iter(&app.world)
            .count(),
        9 * map::star::STARS_PER_SECTOR
    );

    // let (mut c_id, mut c_transform) = app
    //     .world
    //     .query::<(&Camera, &mut Transform)>()
    //     .single_mut(&mut app.world);

    // // Move camera to sector (-1, -1)
    // c_transform.translation.x = -1.0;
    // c_transform.translation.y = -1.0;

    // // Discover six new sectors
    // app.update();

    // assert_eq!(
    //     app.world
    //         .query::<&map::star::Star>()
    //         .iter(&app.world)
    //         .count(),
    //     (9 + 6) * map::star::STARS_PER_SECTOR
    // );
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
