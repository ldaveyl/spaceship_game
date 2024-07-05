use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
        // Could have imported `update_position` function, but it's better to do via sets
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}

// fn print_position(query: Query<(Entity, &Transform, &Velocity)>) {
//     for (entity, transform, velocity) in query.iter() {
//         info!(
//             "Entity {:?} is at position {:?}, with velocity {:?}",
//             entity, transform.translation, velocity
//         );
//     }
// }
