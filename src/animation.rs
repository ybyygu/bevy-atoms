// [[file:../bevy.note::8bf0b235][8bf0b235]]
use bevy::prelude::*;
use bevy::utils::Duration;
// 8bf0b235 ends here

// [[file:../bevy.note::5e188eb0][5e188eb0]]
#[derive(Default, Debug, Clone)]
pub struct AnimationState {
    timer: Timer,
    frames: Vec<usize>,
    frame_index: usize,
}

#[derive(Eq, PartialEq, Default)]
pub enum AnimationMode {
    #[default]
    Loop,
    Once,
    Palindrome,
}

/// Animation controls
#[derive(Component, Default)]
pub struct AnimationPlayer {
    paused: bool,

    mode: AnimationMode,
}

impl AnimationPlayer {
    /// Pause the animation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Unpause the animation
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Is the animation paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

// examples/animation/animated_fox.rs
fn keyboard_animation_control(keyboard_input: Res<Input<KeyCode>>, mut animation_player: Query<&mut AnimationPlayer>) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }
    }
}
// 5e188eb0 ends here

// [[file:../bevy.note::ec994bf0][ec994bf0]]
#[derive(Component)]
pub struct ToggleVisibility(pub Timer);

impl Default for ToggleVisibility {
    fn default() -> Self {
        ToggleVisibility(Timer::new(Duration::from_millis(40), TimerMode::Repeating))
    }
}

fn is_hidden(vis: &Visibility) -> bool {
    matches!(vis, &Visibility::Hidden)
}

pub fn toggle_visibility(time: Res<Time>, mut query: Query<(&mut ToggleVisibility, &mut Visibility)>) {
    for (mut timer, mut visibility) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if is_hidden(&visibility) {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
// ec994bf0 ends here

// [[file:../bevy.note::d116ad14][d116ad14]]
#[derive(Debug, Component)]
pub struct LoopedAnimationTimer(Timer);

impl LoopedAnimationTimer {
    pub fn new(interval: Duration) -> Self {
        LoopedAnimationTimer(Timer::new(interval, TimerMode::Repeating))
    }
}

impl Default for LoopedAnimationTimer {
    fn default() -> Self {
        LoopedAnimationTimer::new(Duration::from_millis(100))
    }
}
// d116ad14 ends here

// [[file:../bevy.note::84e75727][84e75727]]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
// 84e75727 ends here
