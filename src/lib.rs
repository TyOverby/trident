use std::collections::HashMap;

mod state_transition;

pub struct GameState {
    pub stars: HashMap<StarId, Star>,
    pub ships: Vec<ShipGroup>,
    pub carriers: HashMap<CarrierId, Carrier>,
    pub players: HashMap<PlayerId, Player>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StarId(u32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayerId(u32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CarrierId(u32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Player {
    pub id: PlayerId,
    pub cash: u32,
    pub planets_owned: u16,
    pub weapons_research: u8,
    pub terraforming_research: u8,
    pub experimentation_research: u8,
    pub scanning_research: u8,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Star {
    pub id: StarId,
    pub owned_by: PlayerId,
    pub location: (i16, i16),
    pub economy: u8,
    pub infrastructure: u8,
    pub science: u8,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Travel {
    pub from: StarId,
    pub to: StarId,
    pub ticks_to_reach: i16,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ShipGroup {
    pub at: StarId,
    pub owner: PlayerId,
    pub count: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Carrier {
    pub id: CarrierId,
    pub at: Either<StarId, Travel>,
    pub owner: PlayerId,
    pub count: u32,
}

impl GameState {
    pub fn carrier(&self, id: CarrierId) -> Result<&Carrier, String> {
        match self.carriers.get(&id) {
            None => Err(format!(r#"no ship with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }

    pub fn carrier_mut(&mut self, id: CarrierId) -> Result<&mut Carrier, String> {
        match self.carriers.get_mut(&id) {
            None => Err(format!(r#"no ship with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }

    pub fn player(&self, id: PlayerId) -> Result<&Player, String> {
        match self.players.get(&id) {
            None => Err(format!(r#"no player with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }

    pub fn player_mut(&mut self, id: PlayerId) -> Result<&mut Player, String> {
        match self.players.get_mut(&id) {
            None => Err(format!(r#"no player with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }

    pub fn star(&self, id: StarId) -> Result<&Star, String> {
        match self.stars.get(&id) {
            None => Err(format!(r#"no star with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }

    pub fn star_mut(&mut self, id: StarId) -> Result<&mut Star, String> {
        match self.stars.get_mut(&id) {
            None => Err(format!(r#"no star with id "{:?}""#, id)),
            Some(c) => Ok(c)
        }
    }
}
