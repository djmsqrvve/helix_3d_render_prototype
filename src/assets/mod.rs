//! Asset Pipeline Module
//!
//! Asset importing, processing, hot-reload, format conversion.
//! Comparable to Unreal's Content Browser / Asset Pipeline.
//!
//! Future systems:
//! - GLTF/FBX/OBJ import pipeline
//! - Texture compression & processing
//! - Asset dependency tracking
//! - Hot-reload for assets during development
//! - Asset database / registry
//! - Thumbnail generation
//! - Asset validation & error reporting

mod registry;

#[allow(unused_imports)]
pub use registry::AssetRegistry;
