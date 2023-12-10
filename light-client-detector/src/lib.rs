//! The detector component of the light client detects and handles attacks on the light client.
//!
//! See [`detect_divergence`] for the main entry point.

mod conflict;
mod detect;
mod error;
mod evidence;
mod examine;
mod provider;
mod trace;

pub use cometbft::evidence::{Evidence, LightClientAttackEvidence};
pub use conflict::gather_evidence_from_conflicting_headers;
pub use detect::{compare_new_header_with_witness, detect_divergence, CompareError, Divergence};
pub use error::{Error, ErrorDetail};
pub use provider::Provider;
pub use trace::Trace;
