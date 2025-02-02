use crate::{
    audio::Sounds,
    components::{
        BarrierComponent, BlastComponent, BlastType, EnemyComponent, HealthComponent,
        Motion2DComponent, PlayerComponent,
    },
    entities::{spawn_effect, EffectType, EnemyType, SpawnableType},
    events::{EnemyCollisionEvent, PlayAudioEvent},
    resources::{EffectsResource, GameParametersResource, SpriteSheetsResource},
    systems::{barrier_collision, immovable_collision, standard_collision},
};
use amethyst::{
    core::transform::Transform,
    ecs::*,
    ecs::{Read, System, World},
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct EnemyPlayerCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyPlayerCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        Read<'s, GameParametersResource>,
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            enemy_collision_event_channel,
            game_parameters,
            players,
            mut enemies,
            mut motions,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // Is the enemy colliding with an entity with a spaceship component?
            if let Some(player) = players.get(event.colliding_entity) {
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["metal_crash"].clone(),
                });

                let enemy = enemies.get_mut(event.enemy_entity).unwrap();
                let enemy_motion = motions.get_mut(event.enemy_entity).unwrap();
                let enemy_health = healths.get_mut(event.enemy_entity).unwrap();

                match enemy.spawnable_type {
                    SpawnableType::Enemy(EnemyType::Missile) => {
                        enemy_health.value = 0.0;
                    }

                    _ => {
                        enemy_health.value -= player.collision_damage;
                    }
                }

                if !enemy_motion.immovable {
                    if let Some(collision_velocity) = event.collision_velocity {
                        standard_collision(
                            enemy_motion,
                            collision_velocity,
                            game_parameters.min_collision_knockback,
                        );
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemyEnemyCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyEnemyCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        Read<'s, GameParametersResource>,
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            enemy_collision_event_channel,
            game_parameters,
            enemies,
            mut motions,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in enemy_collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(colliding_enemy) = enemies.get(event.colliding_entity) {
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["metal_crash"].clone(),
                });
                let enemy = enemies.get(event.enemy_entity).unwrap();
                let enemy_motion = motions.get_mut(event.enemy_entity).unwrap();
                let enemy_health = healths.get_mut(event.enemy_entity).unwrap();

                match enemy.spawnable_type {
                    SpawnableType::Enemy(EnemyType::Missile) => {
                        enemy_health.value = 0.0;
                    }

                    _ => {
                        enemy_health.value -= colliding_enemy.collision_damage;
                    }
                }

                if !enemy_motion.immovable {
                    if let Some(collision_velocity) = event.collision_velocity {
                        if event.collider_immovable {
                            immovable_collision(
                                enemy_motion,
                                collision_velocity,
                                game_parameters.min_collision_knockback,
                            );
                        } else {
                            standard_collision(
                                enemy_motion,
                                collision_velocity,
                                game_parameters.min_collision_knockback,
                            );
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemyBlastCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyBlastCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        Entities<'s>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlastComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, EffectsResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_channel,
            entities,
            mut healths,
            mut blasts,
            transforms,
            effects_resource,
            sprite_resource,
            lazy_update,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Some(blast) = blasts.get_mut(event.colliding_entity) {
                let enemy_health = healths.get_mut(event.enemy_entity).unwrap();
                let blast_transform = transforms.get(event.colliding_entity).unwrap();

                match blast.blast_type {
                    BlastType::Ally | BlastType::AllyCritical | BlastType::AllyPoison => {
                        entities
                            .delete(event.colliding_entity)
                            .expect("unable to delete entity");

                        play_audio_channel.single_write(PlayAudioEvent {
                            source: sounds.sound_effects["metal_ping"].clone(),
                        });

                        spawn_effect(
                            match blast.blast_type {
                                BlastType::Ally => &EffectType::AllyBlastExplosion,
                                BlastType::AllyCritical => &EffectType::CriticalBlastExplosion,
                                BlastType::AllyPoison => &EffectType::PoisonBlastExplosion,
                                _ => {
                                    panic!("unreachable")
                                }
                            },
                            blast_transform.clone(),
                            &effects_resource,
                            &sprite_resource,
                            &entities,
                            &lazy_update,
                        );

                        enemy_health.value -= blast.damage;
                        //TODO: apply poison to enemy health component from blast
                        //enemy.poison = blast.poison_damage;
                    }

                    _ => {}
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemyArenaBorderCollisionSystem {
    event_reader: Option<ReaderId<EnemyCollisionEvent>>,
}

impl<'s> System<'s> for EnemyArenaBorderCollisionSystem {
    type SystemData = (
        Read<'s, EventChannel<EnemyCollisionEvent>>,
        ReadStorage<'s, BarrierComponent>,
        ReadStorage<'s, EnemyComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyCollisionEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            collision_event_channel,
            barriers,
            enemies,
            mut motion_2ds,
            mut healths,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
            // is the enemy colliding with a barrier?
            if let Some(barrier) = barriers.get(event.colliding_entity) {
                let enemy = enemies.get(event.enemy_entity).unwrap();

                if !barrier.enemies_pass {
                    match enemy.spawnable_type {
                        SpawnableType::Enemy(EnemyType::Missile) => {}

                        _ => {
                            let enemy_motion = motion_2ds.get_mut(event.enemy_entity).unwrap();
                            let enemy_health = healths.get_mut(event.enemy_entity).unwrap();

                            barrier_collision(enemy_motion, barrier);

                            enemy_health.value -= barrier.damage;

                            play_audio_channel.single_write(PlayAudioEvent {
                                source: sounds.sound_effects["force_field"].clone(),
                            });
                        }
                    }
                }
            }
        }
    }
}
