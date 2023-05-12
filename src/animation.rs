// [[file:../bevy.note::8bf0b235][8bf0b235]]
use bevy::prelude::*;
use bevy::utils::Duration;
// 8bf0b235 ends here

// [[file:../bevy.note::5e188eb0][5e188eb0]]
#[derive(Eq, PartialEq, Default)]
pub enum AnimationMode {
    #[default]
    Loop,
    Once,
    Palindrome,
}

#[derive(Resource)]
pub struct AnimationPlayer {
    /// Current frame index when play animation. For simplicity
    /// without considering num of frames, we allow `current_frame` to
    /// be negative or larger than the number of frames.
    current_frame: isize,
    timer: Timer,
    mode: AnimationMode,
}
// 5e188eb0 ends here

// [[file:../bevy.note::73b1c5cd][73b1c5cd]]
impl AnimationPlayer {
    pub fn new(interval: f32) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            current_frame: 0,
            mode: AnimationMode::default(),
        }
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.timer.pause();
    }

    /// Unpause the animation
    pub fn resume(&mut self) {
        self.timer.unpause();
    }

    /// Is the animation paused
    pub fn is_paused(&self) -> bool {
        self.timer.paused()
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new(0.2)
    }
}
// 73b1c5cd ends here

// [[file:../bevy.note::439d4eea][439d4eea]]
// examples/animation/animated_fox.rs
fn keyboard_animation_control(keyboard_input: Res<Input<KeyCode>>, mut player: ResMut<AnimationPlayer>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if player.is_paused() {
            player.resume();
        } else {
            player.timer.pause();
        }
    }
}

fn play_animation(time: Res<Time>, mut player: ResMut<AnimationPlayer>) {
    if !player.timer.tick(time.delta()).just_finished() {
        player.current_frame += 1;
    };
}
// 439d4eea ends here

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

// [[file:../bevy.note::84e75727][84e75727]]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
// 84e75727 ends here
