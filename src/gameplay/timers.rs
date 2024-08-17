use super::*;

#[derive(Debug, Resource)]
pub struct ManualMove(pub Timer);

#[derive(Debug, Resource)]
pub struct AutoMove(pub Timer);

#[derive(Debug, Resource)]
pub struct RemovePieceComponent(pub Timer);
