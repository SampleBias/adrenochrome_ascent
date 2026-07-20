//! Serum debuff + Injector counter (TODO-027).

use bevy::prelude::*;

use super::vitals::{apply_damage, Armor, Health, PainFlash};
use super::Player;

/// Debuff from Serum Zombies / Scientist: darkens vision and drains health.
#[derive(Resource, Debug, Clone, Copy)]
pub struct SerumEffect {
    pub active: bool,
    pub time_left: f32,
    pub drain_per_sec: f32,
    /// 0..1 overlay darkness for UI.
    pub vision_dark: f32,
}

impl Default for SerumEffect {
    fn default() -> Self {
        Self {
            active: false,
            time_left: 0.0,
            drain_per_sec: 6.0,
            vision_dark: 0.0,
        }
    }
}

pub fn apply_serum(serum: &mut SerumEffect, duration: f32) {
    serum.active = true;
    serum.time_left = serum.time_left.max(duration);
    serum.vision_dark = 0.75;
}

/// Clear serum (Adrenochrome Injector counter). Returns true if something was cured.
pub fn cure_serum(serum: &mut SerumEffect) -> bool {
    if !serum.active && serum.vision_dark <= 0.01 {
        return false;
    }
    serum.active = false;
    serum.time_left = 0.0;
    serum.vision_dark = 0.0;
    true
}

pub fn tick_serum_effect(
    time: Res<Time>,
    mut serum: ResMut<SerumEffect>,
    mut pain: ResMut<PainFlash>,
    mut player: Query<(&mut Health, &mut Armor), With<Player>>,
) {
    let dt = time.delta_secs();
    if serum.active {
        serum.time_left -= dt;
        serum.vision_dark = (serum.time_left / 8.0).clamp(0.25, 0.85);
        if let Ok((mut health, mut armor)) = player.single_mut() {
            apply_damage(&mut health, &mut armor, serum.drain_per_sec * dt * 0.4);
            health.current = (health.current - serum.drain_per_sec * dt * 0.6).max(1.0);
        }
        if (time.elapsed_secs() * 2.0) as i32 % 2 == 0 {
            pain.trigger(0.1);
        }
        if serum.time_left <= 0.0 {
            serum.active = false;
            serum.time_left = 0.0;
        }
    } else if serum.vision_dark > 0.0 {
        serum.vision_dark = (serum.vision_dark - dt * 0.5).max(0.0);
    }
}

/// Teal/cyan fullscreen veil while serum-affected.
#[derive(Component)]
pub struct SerumOverlayUi;

pub fn sync_serum_overlay_ui(
    serum: Res<SerumEffect>,
    mut commands: Commands,
    mut existing: Query<(Entity, &mut BackgroundColor), With<SerumOverlayUi>>,
) {
    if let Ok((entity, mut bg)) = existing.single_mut() {
        if serum.vision_dark <= 0.01 {
            commands.entity(entity).despawn();
        } else {
            *bg = BackgroundColor(Color::srgba(0.05, 0.25, 0.35, serum.vision_dark * 0.55));
        }
        return;
    }
    if serum.vision_dark <= 0.01 {
        return;
    }
    commands.spawn((
        SerumOverlayUi,
        Name::new("SerumOverlay"),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.25, 0.35, serum.vision_dark * 0.55)),
        GlobalZIndex(90),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injector_cures_serum() {
        let mut s = SerumEffect::default();
        apply_serum(&mut s, 5.0);
        assert!(s.active);
        assert!(cure_serum(&mut s));
        assert!(!s.active);
        assert!(s.vision_dark < 0.01);
    }
}
