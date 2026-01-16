use std::time::Duration;

use bevy::prelude::*;
use bevy_firefly::lights::PointLight2d;
use bevy_tweening::{Lens, Tween, TweenAnim};
use rand::prelude::*;

use crate::game::{
    map::{GRID_SIZE, inerts::torch::Torch},
    tick::{MainTick, TickDelta},
};

const RANGE_GRID_COUNT: f32 = 2.;
const FLICKER_STEPS: [f32; 7] = [0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];

impl super::LightParameters for Torch {
    fn range() -> f32 {
        GRID_SIZE as f32 * RANGE_GRID_COUNT
    }

    fn color() -> Color {
        Color::srgb(1.0, 0.95, 0.49)
    }

    fn offset() -> Vec3 {
        vec3(0., 6., 0.)
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (super::add_lights::<Torch>, flicker));
}

struct PointLight2dRangeLens {
    start: f32,
    end: f32,
}

impl Lens<PointLight2d> for PointLight2dRangeLens {
    fn lerp(&mut self, mut target: Mut<PointLight2d>, ratio: f32) {
        target.range = self.start + (self.end - self.start) * ratio;
    }
}

fn flicker(
    mut commands: Commands,
    main_tick: Res<MainTick>,
    tick_delta: Res<TickDelta>,
    torches: Query<(Entity, &PointLight2d), With<Torch>>,
) {
    if main_tick.timer.just_finished() {
        for (entity, torch) in torches {
            let mut rng = rand::rng();

            if let Some(intensity) = FLICKER_STEPS.choose(&mut rng) {
                let tween = Tween::new(
                    EaseFunction::BounceInOut,
                    Duration::from_secs_f32(tick_delta.note),
                    PointLight2dRangeLens {
                        start: torch.range,
                        end: GRID_SIZE as f32 * RANGE_GRID_COUNT * intensity,
                    },
                );

                commands.entity(entity).insert(TweenAnim::new(tween));
            }
        }
    }
}
