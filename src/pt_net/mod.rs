//! Contains functionality related to Place/Transition Petri Nets
//!

pub mod parser;

use core::Id;

/// A petri net based on the Place/Transition net definition.
/// The net conforms to the format defined by http://www.pnml.org/version-2009/grammar/ptnet
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Net {
    pub id: Id,
    pub elements: Vec<Element>,
}

/// An element of a PT net
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Element {
    /// A place with an initial marking
    Place { id: Id, initial_marking: u32 },
    /// A transition
    Transition { id: Id },
    /// An arc defining the relationships between places and transitions
    Arc { id: Id, source: Id, target: Id, inscription: u32 }
}


impl Element {

    pub fn new_place(id: &str, initial_marking: u32) -> Element {
        Element::Place { id: id.to_string(), initial_marking: initial_marking }
    }

    pub fn new_transition(id: &str) -> Element {
        Element::Transition { id: id.to_string() }
    }

    pub fn new_arc(id: &str, source: &str, target: &str, inscription: u32) -> Element {
        Element::Arc {
            id: id.to_string(),
            source: source.to_string(),
            target: target.to_string(),
            inscription: inscription,
        }
    }

    pub fn is_place(&self) -> bool {
        match self {
            &Element::Place { .. } => true,
            _ => false,
        }
    }

    pub fn is_transition(&self) -> bool {
        match self {
            &Element::Transition { .. } => true,
            _ => false,
        }
    }

    pub fn is_arc(&self) -> bool {
        match self {
            &Element::Arc { .. } => true,
            _ => false,
        }
    }

}
