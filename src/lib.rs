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
//!   Pre-built eras from Bronze Age to Information Age, `eras_containing(year)` lookup
//! - [`civilization`] — Major civilizations with geographic extent, peak period,
//!   key traits. 10 pre-built civilizations, `by_region()` and `active_at()` lookups
//! - [`event`] — Structured historical events with category, era, and civilizations
//!   involved. 15 pre-built major world events
//! - [`calendar`] — Calendar system metadata: type, epoch, months, leap rules
//!   (not computation — that belongs in sankhya). 8 pre-built calendar systems
//! - [`figure`] — Historical figures with era/civilization context and domain
//!   classification. 10 pre-built figures
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

pub mod calendar;
pub mod civilization;
pub mod era;
pub mod error;
pub mod event;
pub mod figure;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::ItihasError;
