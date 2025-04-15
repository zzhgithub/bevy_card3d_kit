use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Card {
    pub origin: Transform,
}

#[derive(Debug, Clone, Copy, Component, Default)]
pub enum Dragged {
    /// The card is being actively dragged
    #[default]
    Actively,
    /// The card is no longer being dragged and is currently going back to its origin place
    GoingBackToPlace,
}

/// A tag added to hovered cards, indicating that they're hovered over
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Hovered;
