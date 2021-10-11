use bevy::ecs::{entity::Entity as BevyEntity, system::{Query, ResMut}, query::With};

use naia_bevy_client::{Client, components::Predicted, Entity};

use naia_bevy_demo_shared::protocol::Protocol;

use crate::resources::Global;

pub fn tick(
    mut client: Client<Protocol>,
    mut global: ResMut<Global>,
    q_player_position: Query<BevyEntity, With<Predicted>>) {
    // All game logic should happen here, on a tick event
    //info!("tick");

    if let Ok(entity) = q_player_position.single() {
        if let Some(command) = global.queued_command.take() {
            client.queue_command(&Entity::new(entity), &command);
        }
    }
}
