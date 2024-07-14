#![no_std]
//! hello
use gmeta::{In, InOut, Metadata, Out};
use gstd::prelude::*;
use gstd::{Decode, Encode}; // Import necessary functions
use scale_info::TypeInfo; // Import TypeInfo derive macro (assuming you're using scale-info)


pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
/// pub struct PebblesInit 
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

/// another function
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Player {
    #[default]
    User,
    Program,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}