use crate::{
    components::{Animation, AnimationType, BlastType, TimeLimitComponent},
    resources::SpriteResource,
};
use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn spawn_explosion(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteResource>,
    sprite_number: usize,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let frame_time: f32 = 0.1;
    let frame_count: usize = 10;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.explosions_sprite_sheet.clone(),
        sprite_number,
    };

    let animation = Animation {
        start_idx: 0,
        frame_count,
        current_frame: 0,
        frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    let named = Named::new("explosion");

    let timed = TimeLimitComponent { duration };

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite)
        .with(animation)
        .with(local_transform)
        .with(named)
        .with(timed)
        .build()
}

pub fn spawn_blast_explosion(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    blast_type: BlastType,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let frame_time: f32 = 0.08;
    let frame_count: usize = 7;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let starting_frame: usize = match blast_type {
        BlastType::Player => 0,
        BlastType::Enemy => 7,
        BlastType::Critical => 14,
        BlastType::Poison => 21,
    };

    let sprite = SpriteRender {
        sprite_sheet,
        sprite_number: starting_frame,
    };

    let animation = Animation {
        start_idx: starting_frame,
        frame_count,
        current_frame: starting_frame,
        frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    let named = Named::new("blast_explosion");

    let timed = TimeLimitComponent { duration };

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite)
        .with(animation)
        .with(local_transform)
        .with(named)
        .with(timed)
        .build()
}
