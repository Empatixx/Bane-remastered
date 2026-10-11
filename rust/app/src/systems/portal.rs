use hecs::World;

use crate::components::{Player, Portal, Position, Size};
use crate::geometry;

pub(crate) fn player_in_portal(world: &World) -> bool {
    world
        .query::<(&Position, &Size)>()
        .with::<&Player>()
        .iter()
        .any(|(player_position, player_size)| {
            world
                .query::<(&Position, &Size)>()
                .with::<&Portal>()
                .iter()
                .any(|(portal_position, portal_size)| {
                    geometry::overlaps(
                        player_position.0,
                        player_size.0,
                        portal_position.0,
                        portal_size.0,
                    )
                })
        })
}
