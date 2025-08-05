//! Asset management system for the Lumina Editor
//! 
//! This module provides functionality for importing, organizing,
//! and managing game assets like images, sounds, and scripts.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Represents a game asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAsset {
    /// Unique identifier
    pub id: Uuid,
    /// Asset name (filename without extension)
    pub name: String,
    /// Full file path
    pub path: PathBuf,
    /// Asset type
    pub asset_type: AssetType,
    /// File size in bytes
    pub size: u64,
    /// Asset metadata
    pub metadata: AssetMetadata,
    /// Asset tags for organization
    pub tags: Vec<String>,
}

/// Types of assets supported by the engine
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AssetType {
    /// Image files (PNG, JPG, GIF, etc.)
    Image,
    /// Audio files (WAV, MP3, OGG, etc.)
    Audio,
    /// Font files (TTF, OTF, etc.)
    Font,
    /// Script files (Lua, etc.)
    Script,
    /// Scene files
    Scene,
    /// Custom data files (JSON, TOML, etc.)
    Data,
    /// 3D model files (eventually)
    Model,
    /// Unknown file type
    Unknown,
}

impl AssetType {
    /// Get the display name for the asset type
    pub fn display_name(&self) -> &str {
        match self {
            AssetType::Image => "Image",
            AssetType::Audio => "Audio", 
            AssetType::Font => "Font",
            AssetType::Script => "Script",
            AssetType::Scene => "Scene",
            AssetType::Data => "Data",
            AssetType::Model => "Model",
            AssetType::Unknown => "Unknown",
        }
    }

    /// Get the color for this asset type (for UI visualization)
    pub fn color(&self) -> [f32; 4] {
        match self {
            AssetType::Image => [0.2, 0.8, 0.2, 1.0],      // Green
            AssetType::Audio => [0.8, 0.2, 0.8, 1.0],      // Purple
            AssetType::Font => [0.2, 0.2, 0.8, 1.0],       // Blue
            AssetType::Script => [0.8, 0.8, 0.2, 1.0],     // Yellow
            AssetType::Scene => [0.2, 0.8, 0.8, 1.0],      // Cyan
            AssetType::Data => [0.8, 0.5, 0.2, 1.0],       // Orange
            AssetType::Model => [0.6, 0.3, 0.8, 1.0],      // Violet
            AssetType::Unknown => [0.5, 0.5, 0.5, 1.0],    // Gray
        }
    }

    /// Determine asset type from file extension
    pub fn from_extension(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => AssetType::Image,
            "wav" | "mp3" | "ogg" | "flac" | "aac" => AssetType::Audio,
            "ttf" | "otf" | "woff" | "woff2" => AssetType::Font,
            "lua" | "js" | "py" | "rs" => AssetType::Script,
            "scene" | "scn" => AssetType::Scene,
            "json" | "toml" | "yaml" | "yml" | "xml" => AssetType::Data,
            "obj" | "fbx" | "gltf" | "glb" => AssetType::Model,
            _ => AssetType::Unknown,
        }
    }

    /// Get supported file extensions for this asset type
    pub fn extensions(&self) -> &[&str] {
        match self {
            AssetType::Image => &["png", "jpg", "jpeg", "gif", "bmp", "webp"],
            AssetType::Audio => &["wav", "mp3", "ogg", "flac", "aac"],
            AssetType::Font => &["ttf", "otf", "woff", "woff2"],
            AssetType::Script => &["lua", "js", "py", "rs"],
            AssetType::Scene => &["scene", "scn"],
            AssetType::Data => &["json", "toml", "yaml", "yml", "xml"],
            AssetType::Model => &["obj", "fbx", "gltf", "glb"],
            AssetType::Unknown => &[],
        }
    }
}

/// Asset metadata for additional information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// When the asset was imported
    pub imported_at: u64,
    /// When the file was last modified
    pub modified_at: u64,
    /// Import settings used
    pub import_settings: HashMap<String, String>,
    /// Asset dimensions (for images)
    pub dimensions: Option<(u32, u32)>,
    /// Duration (for audio/video)
    pub duration: Option<f32>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl GameAsset {
    /// Create a new asset from a file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let asset_type = AssetType::from_extension(extension);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            path: path.to_path_buf(),
            asset_type,
            size: metadata.len(),
            metadata: AssetMetadata {
                imported_at: now,
                modified_at: metadata.modified()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                import_settings: HashMap::new(),
                dimensions: None,
                duration: None,
                properties: HashMap::new(),
            },
            tags: Vec::new(),
        })
    }

    /// Get the file extension
    pub fn extension(&self) -> Option<&str> {
        self.path.extension().and_then(|s| s.to_str())
    }

    /// Check if the asset file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get human-readable file size
    pub fn size_string(&self) -> String {
        let size = self.size as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.1} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Add a tag to the asset
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove a tag from the asset
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Check if asset has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

/// Asset database for managing all assets in a project
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDatabase {
    /// All assets indexed by ID
    pub assets: HashMap<Uuid, GameAsset>,
    /// Assets organized by type
    pub by_type: HashMap<AssetType, Vec<Uuid>>,
    /// Assets organized by tags
    pub by_tag: HashMap<String, Vec<Uuid>>,
    /// Search index for fast searching
    pub search_index: HashMap<String, Vec<Uuid>>,
}

impl AssetDatabase {
    /// Create a new empty asset database
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            by_type: HashMap::new(),
            by_tag: HashMap::new(),
            search_index: HashMap::new(),
        }
    }

    /// Add an asset to the database
    pub fn add_asset(&mut self, asset: GameAsset) -> Uuid {
        let id = asset.id;
        
        // Add to type index
        self.by_type
            .entry(asset.asset_type.clone())
            .or_insert_with(Vec::new)
            .push(id);

        // Add to tag indices
        for tag in &asset.tags {
            self.by_tag
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(id);
        }

        // Add to search index
        let search_terms = vec![
            asset.name.to_lowercase(),
            asset.asset_type.display_name().to_lowercase(),
        ];

        for term in search_terms {
            self.search_index
                .entry(term)
                .or_insert_with(Vec::new)
                .push(id);
        }

        // Add to main storage
        self.assets.insert(id, asset);
        
        log::info!("Added asset {} to database", id);
        id
    }

    /// Remove an asset from the database
    pub fn remove_asset(&mut self, id: Uuid) -> Option<GameAsset> {
        if let Some(asset) = self.assets.remove(&id) {
            // Remove from type index
            if let Some(type_list) = self.by_type.get_mut(&asset.asset_type) {
                type_list.retain(|&x| x != id);
            }

            // Remove from tag indices
            for tag in &asset.tags {
                if let Some(tag_list) = self.by_tag.get_mut(tag) {
                    tag_list.retain(|&x| x != id);
                }
            }

            // Remove from search index
            for (_, id_list) in self.search_index.iter_mut() {
                id_list.retain(|&x| x != id);
            }

            log::info!("Removed asset {} from database", id);
            Some(asset)
        } else {
            None
        }
    }

    /// Get an asset by ID
    pub fn get_asset(&self, id: Uuid) -> Option<&GameAsset> {
        self.assets.get(&id)
    }

    /// Get all assets of a specific type
    pub fn get_assets_by_type(&self, asset_type: &AssetType) -> Vec<&GameAsset> {
        self.by_type
            .get(asset_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.assets.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all assets with a specific tag
    pub fn get_assets_by_tag(&self, tag: &str) -> Vec<&GameAsset> {
        self.by_tag
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.assets.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search for assets by name or type
    pub fn search_assets(&self, query: &str) -> Vec<&GameAsset> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        // Exact matches first
        if let Some(ids) = self.search_index.get(&query) {
            for id in ids {
                if let Some(asset) = self.assets.get(id) {
                    results.push(asset);
                }
            }
        }

        // Partial matches
        for (term, ids) in &self.search_index {
            if term.contains(&query) && term != &query {
                for id in ids {
                    if let Some(asset) = self.assets.get(id) {
                        if !results.iter().any(|a| a.id == asset.id) {
                            results.push(asset);
                        }
                    }
                }
            }
        }

        results
    }

    /// Get all assets
    pub fn get_all_assets(&self) -> Vec<&GameAsset> {
        self.assets.values().collect()
    }

    /// Import assets from a directory
    pub fn import_from_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<Vec<Uuid>> {
        let dir = dir.as_ref();
        let mut imported = Vec::new();

        if !dir.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {:?}", dir));
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match GameAsset::from_path(&path) {
                    Ok(asset) => {
                        let id = self.add_asset(asset);
                        imported.push(id);
                    }
                    Err(e) => {
                        log::warn!("Failed to import asset {:?}: {}", path, e);
                    }
                }
            }
        }

        log::info!("Imported {} assets from {:?}", imported.len(), dir);
        Ok(imported)
    }

    /// Save the database to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load the database from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let database: AssetDatabase = serde_json::from_str(&content)?;
        Ok(database)
    }

    /// Get statistics about the database
    pub fn get_stats(&self) -> AssetStats {
        let mut stats = AssetStats {
            total_assets: self.assets.len(),
            total_size: self.assets.values().map(|a| a.size).sum(),
            by_type: HashMap::new(),
        };

        for asset in self.assets.values() {
            let type_stats = stats.by_type.entry(asset.asset_type.clone()).or_insert(TypeStats {
                count: 0,
                size: 0,
            });
            type_stats.count += 1;
            type_stats.size += asset.size;
        }

        stats
    }
}

impl Default for AssetDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about assets in the database
#[derive(Debug)]
pub struct AssetStats {
    pub total_assets: usize,
    pub total_size: u64,
    pub by_type: HashMap<AssetType, TypeStats>,
}

/// Statistics for a specific asset type
#[derive(Debug)]
pub struct TypeStats {
    pub count: usize,
    pub size: u64,
}

impl AssetStats {
    /// Get human-readable total size
    pub fn total_size_string(&self) -> String {
        let size = self.total_size as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.1} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Asset browser state and filtering
#[derive(Debug)]
pub struct AssetBrowser {
    /// The asset database
    pub database: AssetDatabase,
    /// Current search query
    pub search_query: String,
    /// Current type filter
    pub type_filter: Option<AssetType>,
    /// Current tag filter
    pub tag_filter: Option<String>,
    /// Currently selected asset
    pub selected_asset: Option<Uuid>,
}

impl AssetBrowser {
    /// Create a new asset browser
    pub fn new() -> Self {
        Self {
            database: AssetDatabase::new(),
            search_query: String::new(),
            type_filter: None,
            tag_filter: None,
            selected_asset: None,
        }
    }

    /// Get filtered assets based on current filters
    pub fn get_filtered_assets(&self) -> Vec<&GameAsset> {
        let mut assets = if self.search_query.is_empty() {
            self.database.get_all_assets()
        } else {
            self.database.search_assets(&self.search_query)
        };

        // Apply type filter
        if let Some(type_filter) = &self.type_filter {
            assets.retain(|asset| &asset.asset_type == type_filter);
        }

        // Apply tag filter
        if let Some(tag_filter) = &self.tag_filter {
            assets.retain(|asset| asset.has_tag(tag_filter));
        }

        assets
    }

    /// Set the search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    /// Set the type filter
    pub fn set_type_filter(&mut self, asset_type: Option<AssetType>) {
        self.type_filter = asset_type;
    }

    /// Set the tag filter
    pub fn set_tag_filter(&mut self, tag: Option<String>) {
        self.tag_filter = tag;
    }

    /// Select an asset
    pub fn select_asset(&mut self, asset_id: Option<Uuid>) {
        self.selected_asset = asset_id;
        if let Some(id) = asset_id {
            log::debug!("Selected asset: {}", id);
        } else {
            log::debug!("Cleared asset selection");
        }
    }

    /// Get the currently selected asset
    pub fn get_selected_asset(&self) -> Option<&GameAsset> {
        self.selected_asset.and_then(|id| self.database.get_asset(id))
    }
}

impl Default for AssetBrowser {
    fn default() -> Self {
        Self::new()
    }
}