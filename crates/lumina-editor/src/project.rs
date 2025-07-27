//! Editor project management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Represents an editor project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorProject {
    /// Project unique identifier
    pub id: Uuid,
    /// Project name
    pub name: String,
    /// Project root path
    pub path: PathBuf,
    /// Project version
    pub version: String,
    /// Project metadata
    pub metadata: ProjectMetadata,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project description
    pub description: String,
    /// Project author
    pub author: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub modified_at: u64,
    /// Engine version this project was created with
    pub engine_version: String,
}

impl EditorProject {
    /// Create a new project
    pub fn new(name: String, path: String) -> Result<Self> {
        let project_path = PathBuf::from(path);
        
        // Create project directory if it doesn't exist
        if !project_path.exists() {
            std::fs::create_dir_all(&project_path)?;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let project = Self {
            id: Uuid::new_v4(),
            name: name.clone(),
            path: project_path,
            version: "0.1.0".to_string(),
            metadata: ProjectMetadata {
                description: format!("Lumina project: {}", name),
                author: "User".to_string(), // TODO: Get from system/config
                created_at: now,
                modified_at: now,
                engine_version: "0.1.0".to_string(),
            },
        };
        
        // Save project file
        project.save()?;
        
        log::info!("Created new project '{}' at {:?}", name, project.path);
        Ok(project)
    }
    
    /// Load an existing project
    pub fn load(path: String) -> Result<Self> {
        let project_path = PathBuf::from(path);
        let project_file = project_path.join("project.json");
        
        if !project_file.exists() {
            return Err(anyhow::anyhow!("Project file not found: {:?}", project_file));
        }
        
        let contents = std::fs::read_to_string(project_file)?;
        let project: EditorProject = serde_json::from_str(&contents)?;
        
        log::info!("Loaded project '{}' from {:?}", project.name, project.path);
        Ok(project)
    }
    
    /// Save the project to disk
    pub fn save(&self) -> Result<()> {
        let project_file = self.path.join("project.json");
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(project_file, contents)?;
        
        log::debug!("Saved project '{}'", self.name);
        Ok(())
    }
    
    /// Update the last modified timestamp
    pub fn touch(&mut self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        self.metadata.modified_at = now;
        self.save()
    }
    
    /// Get the project assets directory
    pub fn assets_dir(&self) -> PathBuf {
        self.path.join("assets")
    }
    
    /// Get the project scenes directory
    pub fn scenes_dir(&self) -> PathBuf {
        self.path.join("scenes")
    }
    
    /// Get the project scripts directory
    pub fn scripts_dir(&self) -> PathBuf {
        self.path.join("scripts")
    }
    
    /// Initialize project directory structure
    pub fn init_directory_structure(&self) -> Result<()> {
        // Create standard directories
        std::fs::create_dir_all(self.assets_dir())?;
        std::fs::create_dir_all(self.scenes_dir())?;
        std::fs::create_dir_all(self.scripts_dir())?;
        
        // Create README
        let readme_path = self.path.join("README.md");
        let readme_content = format!(
            "# {}\n\n{}\n\nCreated with Lumina Engine v{}\n",
            self.name,
            self.metadata.description,
            self.metadata.engine_version
        );
        std::fs::write(readme_path, readme_content)?;
        
        log::info!("Initialized directory structure for project '{}'", self.name);
        Ok(())
    }
}