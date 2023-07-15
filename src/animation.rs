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
    /// Total number of frames
    nframes: usize,
    timer: Timer,
    mode: AnimationMode,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct Frame(pub usize);
// 5e188eb0 ends here

// [[file:../bevy.note::73b1c5cd][73b1c5cd]]
impl AnimationPlayer {
    pub fn new(nframes: usize, interval: f32) -> Self {
        assert_ne!(nframes, 0);
        Self {
            nframes,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
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

fn play_animation(
    time: Res<Time>,
    mut player: ResMut<AnimationPlayer>,
    mut current_frame: Local<usize>,
    mut visibility_query: Query<(&mut Visibility, &Frame)>,
) {
    if !player.timer.tick(time.delta()).just_finished() {
        // increase frame
        *current_frame += 1;
        // toggle visibility
        let ci = *current_frame % player.nframes;
        for (mut visibility, Frame(fi)) in visibility_query.iter_mut() {
            if *fi == dbg!(ci) {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
// 439d4eea ends here

// [[file:../bevy.note::84e75727][84e75727]]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationPlayer::new(1, 0.2))
            .add_system(keyboard_animation_control)
            .add_system(play_animation);
    }
}
// 84e75727 ends here
