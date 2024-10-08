use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{GameAssets, GameState};

use super::Player;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let collider = commands
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -12.0, 0.0,
            ))),
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            animator,
            SpriteBundle {
                texture: assets.player_texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: assets.player_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
