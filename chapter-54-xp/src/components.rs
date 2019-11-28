extern crate specs;
use specs::prelude::*;
extern crate specs_derive;
extern crate rltk;
use rltk::{RGB};
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use std::collections::HashMap;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order : i32
}
 
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Bystander {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Vendor {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Name {
    pub name : String
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pool {
    pub max: i32,
    pub current: i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Pools {
    pub hit_points : Pool,
    pub mana : Pool,
    pub xp : i32,
    pub level : i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub base : i32,
    pub modifiers : i32,
    pub bonus : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Attributes {
    pub might : Attribute,
    pub fitness : Attribute,
    pub quickness : Attribute,
    pub intelligence : Attribute
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Skill { Melee, Defense, Magic }

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Skills {
    pub skills : HashMap<Skill, i32>
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct SufferDamage {
    pub amount : i32,
    pub from_player: bool
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct LootTable {
    pub table : String
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Carnivore {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Herbivore {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Ranged {
    pub range : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct InflictsDamage {
    pub damage : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct AreaOfEffect {
    pub radius : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Confusion {
    pub turns : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct ProvidesHealing {
    pub heal_amount : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksVisibility {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Door { 
    pub open: bool 
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct InBackpack {
    pub owner : Entity
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct WantsToPickupItem {
    pub collected_by : Entity,
    pub item : Entity
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct WantsToUseItem {
    pub item : Entity,
    pub target : Option<rltk::Point>
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct WantsToDropItem {
    pub item : Entity
}

// See wrapper below for serialization
#[derive(Component, Debug)]
pub struct WantsToRemoveItem {
    pub item : Entity
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot { Melee, Shield, Head, Torso, Legs, Feet, Hands }

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equippable {
    pub slot : EquipmentSlot
}

// See wrapper below for serialization
#[derive(Component)]
pub struct Equipped {
    pub owner : Entity,
    pub slot : EquipmentSlot
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum WeaponAttribute { Might, Quickness }

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct MeleeWeapon {
    pub attribute : WeaponAttribute,
    pub damage_n_dice : i32,
    pub damage_die_type : i32,
    pub damage_bonus : i32,
    pub hit_bonus : i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Wearable {
    pub armor_class : f32,
    pub slot : EquipmentSlot
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NaturalAttack {
    pub name : String,
    pub damage_n_dice : i32,
    pub damage_die_type : i32,
    pub damage_bonus : i32,
    pub hit_bonus : i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct NaturalAttackDefense {
    pub armor_class : Option<i32>,
    pub attacks : Vec<NaturalAttack>
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms : f32
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum HungerState { WellFed, Normal, Hungry, Starving }

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HungerClock {
    pub state : HungerState,
    pub duration : i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct ProvidesFood {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Hidden {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntryTrigger {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct EntityMoved {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct SingleActivation {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Quips {
    pub available : Vec<String>
}

// Serialization helper code. We need to implement ConvertSaveLoad for each type that contains an
// Entity.

pub struct SerializeMe;

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map : super::map::Map
}

// WantsToMelee wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct WantsToMeleeData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToMelee
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToMeleeData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.target).unwrap();
        Ok(WantsToMeleeData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = ids(data.0).unwrap();
        Ok(WantsToMelee{target: entity})
    }
}

// InBackpack wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct InBackpackData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for InBackpack
where
    for<'de> M: Deserialize<'de>,
{
    type Data = InBackpackData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.owner).unwrap();
        Ok(InBackpackData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = ids(data.0).unwrap();
        Ok(InBackpack{owner: entity})
    }
}

// WantsToPickupItem wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct WantsToPickupItemData<M>(M, M);

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToPickupItem
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToPickupItemData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.collected_by).unwrap();
        let marker2 = ids(self.item).unwrap();
        Ok(WantsToPickupItemData(marker, marker2))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let collected_by = ids(data.0).unwrap();
        let item = ids(data.1).unwrap();
        Ok(WantsToPickupItem{collected_by, item})
    }
}

// WantsToUseItem wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct WantsToUseItemData<M>(M, Option<rltk::Point>);

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToUseItem
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToUseItemData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.item).unwrap();
        Ok(WantsToUseItemData(marker, self.target))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let item = ids(data.0).unwrap();
        let target = data.1;
        Ok(WantsToUseItem{item, target})
    }
}

// WantsToDropItem wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct WantsToDropItemData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToDropItem
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToDropItemData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.item).unwrap();
        Ok(WantsToDropItemData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = ids(data.0).unwrap();
        Ok(WantsToDropItem{item: entity})
    }
}

// WantsToRemoveItem wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct WantsToRemoveItemData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToRemoveItem
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToRemoveItemData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.item).unwrap();
        Ok(WantsToRemoveItemData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = ids(data.0).unwrap();
        Ok(WantsToRemoveItem{item: entity})
    }
}

// Equipped wrapper
#[derive(Serialize, Deserialize, Clone)]
pub struct EquippedData<M>(M, EquipmentSlot);

impl<M: Marker + Serialize> ConvertSaveload<M> for Equipped
where
    for<'de> M: Deserialize<'de>,
{
    type Data = EquippedData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = ids(self.owner).unwrap();
        Ok(EquippedData(marker, self.slot))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = ids(data.0).unwrap();
        Ok(Equipped{owner: entity, slot : data.1})
    }
}