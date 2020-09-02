use crate::components::{Consumable, Enemy, Hitbox2DComponent, Item};
use serde::{Deserialize, Serialize};

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub item_component: Item,
    pub hitbox_component: Hitbox2DComponent,
}

pub type EnemyPool = std::collections::HashMap<String, Enemy>;
pub type ItemPool = std::collections::HashMap<String, ItemEntityData>;
pub type ConsumablePool = std::collections::HashMap<String, Consumable>;
