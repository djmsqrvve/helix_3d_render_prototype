#![allow(dead_code)]
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// Central asset registry — tracks all loaded assets and their metadata
#[derive(Resource, Default, Debug)]
pub struct AssetRegistry {
    /// Map of asset name -> asset info
    pub entries: HashMap<String, AssetEntry>,
    /// Directory roots to scan for assets
    pub asset_roots: Vec<PathBuf>,
}

/// Metadata for a single asset
#[derive(Debug, Clone)]
pub struct AssetEntry {
    /// Display name
    pub name: String,
    /// File path relative to asset root
    pub path: PathBuf,
    /// Asset type classification
    pub asset_type: AssetType,
    /// Whether this asset is currently loaded in memory
    pub is_loaded: bool,
    /// File size in bytes
    pub file_size: u64,
}

/// Classification of asset types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Model,
    Texture,
    Animation,
    Material,
    Audio,
    Scene,
    Script,
    Other,
}

// Future: implement scan_assets, load_asset, hot_reload_watcher, etc.
