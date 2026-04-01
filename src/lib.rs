//! Itihas — Structured World History
//!
//! **Itihas** (Sanskrit: इतिहास — "thus it was", history, chronicle) provides
//! structured, queryable world history data. Civilizations, eras, events,
//! historical figures, and calendar system metadata as Rust types.
//!
//! # Architecture
//!
//! Six modules:
//!
//! - [`era`] — Historical periods with date ranges and civilizational phases.
//!   25 pre-built eras (8 global + 17 regional), `eras_containing(year)` lookup
//! - [`civilization`] — Major civilizations with geographic extent, peak period,
//!   key traits. 52 pre-built civilizations, `by_region()` and `active_at()` lookups
//! - [`event`] — Structured historical events with category, era, significance,
//!   and civilizations involved. 105 pre-built world events
//! - [`calendar`] — Calendar system metadata: type, epoch, months, leap rules
//!   (not computation — that belongs in sankhya). 8 pre-built calendar systems
//! - [`figure`] — Historical figures with era/civilization context and domain
//!   classification. 52 pre-built figures
//! - [`error`] — `ItihasError` with variants for unknown entities and invalid lookups
//!
//! # Relationship to Other Crates
//!
//! ```text
//! itihas (this) — structured world history data
//!   ↓ provides historical context
//! sankhya — ancient mathematical systems (calendar math, era arithmetic)
//!   ↓ computation layer
//! avatara — historical simulation (era transitions, civilization dynamics)
//!   ↓ simulation engine
//! kiran — game engine (historical scenarios, timeline rendering)
//! ```
//!
//! Also feeds:
//! - **joshua** — strategy game (historical civilizations, events)
//! - **jnana** — knowledge system (historical facts, timeline queries)
//! - **lipi** — linguistics (historical script/language context)
//! - **vidya** — programming reference (history of computing)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod calendar;
pub mod civilization;
pub mod era;
pub mod error;
pub mod event;
pub mod figure;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::ItihasError;
