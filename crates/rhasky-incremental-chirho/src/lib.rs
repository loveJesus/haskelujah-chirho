// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-incremental-chirho
//!
//! Incremental compilation and caching for the Rhasky compiler.
//!
//! Provides source fingerprinting, dependency tracking, on-disk artifact
//! caching, and recompilation avoidance. The crate is backend-agnostic:
//! it tracks opaque byte-level artifacts keyed by (module, phase, fingerprint).
//!
//! ## Key concepts
//!
//! - **Fingerprint**: A 128-bit content hash of source text or serialized
//!   intermediate representation.
//! - **ModuleRecord**: Cached metadata for a single module: its source
//!   fingerprint, dependency fingerprints, and per-phase artifact fingerprints.
//! - **IncrementalSession**: The runtime session that checks cache freshness,
//!   stores artifacts, and answers "should I recompile?" queries.
//! - **ArtifactStore**: On-disk key→blob storage backed by a simple directory
//!   layout (`<cache-dir>/<hex-fingerprint>.blob`).

pub mod artifact_chirho;
pub mod dep_chirho;
pub mod fingerprint_chirho;
pub mod session_chirho;

pub use artifact_chirho::ArtifactStoreChirho;
pub use dep_chirho::{DepGraphChirho, DepNodeChirho};
pub use fingerprint_chirho::FingerprintChirho;
pub use session_chirho::{IncrementalSessionChirho, ModuleRecordChirho, PhaseTagChirho, RebuildReasonChirho};
