#![doc = include_str!("../README.md")]

pub use atomics::{AsPtr, AtomicArc, AtomicWeak, SmartPtr, StrongPtr};
pub use snapshot::Snapshot;

pub(crate) mod atomics;
pub(crate) mod snapshot;

/// Traits and structs pertaining to safe memory reclamation.
pub mod smr {
    /// Traits pertaining to deferred reference counting.
    pub mod drc;

    /// The crate-default reclaimer.
    pub mod standard_reclaimer;
}

pub(crate) mod utils {
    pub(crate) mod helpers;
    pub(crate) mod unrolled_linked_list;
    pub(crate) mod unsafe_arc;
}

// Todo:
// Fix all ignored clippy warnings.
