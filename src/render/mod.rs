//! Render Pipeline Module
//!
//! Custom render pipeline, materials, post-processing, and shader management.
//! Comparable to Unreal's Rendering subsystem.
//!
//! Future systems:
//! - Custom material definitions & material editor
//! - Post-processing stack (bloom, tone mapping, SSAO, SSR)
//! - Shader hot-reload
//! - Render passes & render targets
//! - PBR material parameter system

pub mod gpu_info;
mod pipeline;

pub use gpu_info::display_gpu_info;
#[allow(unused_imports)]
pub use pipeline::RenderConfig;
