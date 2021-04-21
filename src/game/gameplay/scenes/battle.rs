use bevy::{prelude::*, render::camera::RenderLayers};

use crate::game::{camera::CustomOrthographicCameraBundle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleLocation {
    Mountains,
}

#[derive(Debug, Clone, Copy)]
pub struct BattleEvent {
    pub battle_location: BattleLocation,
    pub enemy_entity: Entity,
}

pub struct BattleView {
    pub entity: Entity,
}

pub fn get_battle_location_texture(battle_location: BattleLocation) -> &'static str {
    match battle_location {
        BattleLocation::Mountains => "textures/backgrounds/battle1.png",
    }
}

pub fn handle_battle_events(
    mut battle_events: EventReader<BattleEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in battle_events.iter() {
        let texture_handle: Handle<Texture> =
            asset_server.load(get_battle_location_texture(event.battle_location));
        let background_sprite = materials.add(texture_handle.into());

        let battle_entity = commands
            .spawn()
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|child_builder| {
                child_builder.spawn().insert_bundle(SpriteBundle {
                    material: background_sprite,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                }).insert(RenderLayers::layer(1));

                child_builder
                    .spawn()
                    .insert_bundle(CustomOrthographicCameraBundle::new_2d_with_size(Some(
                        Vec2::new(1920.0, 1080.0),
                    )))
                    .insert(RenderLayers::layer(1));
            })
            .id();

        commands.insert_resource(BattleView {
            entity: battle_entity,
        });
    }
}

pub fn clear_battle_screen(mut commands: Commands, battle_view: Res<BattleView>) {
    commands.entity(battle_view.entity).despawn_recursive();
}
