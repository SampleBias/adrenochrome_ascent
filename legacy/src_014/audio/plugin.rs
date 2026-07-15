use bevy::prelude::*;

/// Placeholder audio plugin.
///
/// TODO: add ambient loops, footsteps, detection stingers, puzzle SFX,
/// and UI sounds. Bevy's audio API is used here as a stub.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {
        // No systems yet — audio assets need to be sourced first.
        // When ready:
        //   app.add_systems(Update, play_ambient.run_if(...));
        //   app.add_systems(Update, play_footsteps.run_if(...));
    }
}
