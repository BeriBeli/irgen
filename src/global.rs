use crate::processing::{LoadResult, base};
use gpui::{App, EntityId, Global, ReadGlobal};
use gpui_component::ThemeConfig;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ExportFormat {
    #[default]
    Ipxact,
    Regvue,
    CHeader,
    UvmRal,
    Rtl,
    Html,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeModeSetting {
    #[default]
    System,
    Light,
    Dark,
}

pub struct GlobalState {
    workspace_ids: RwLock<HashSet<EntityId>>,
    component: RwLock<Option<Arc<base::Component>>>,
    directory: RwLock<Option<Arc<PathBuf>>>,
    selected_file: RwLock<Option<Arc<PathBuf>>>,
    selected_file_size: RwLock<Option<u64>>,
    sheet_count: RwLock<Option<usize>>,
    export_format: RwLock<ExportFormat>,
    theme_mode: RwLock<ThemeModeSetting>,
    effective_themes: RwLock<Vec<ThemeConfig>>,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            workspace_ids: RwLock::new(HashSet::new()),
            component: RwLock::new(None),
            directory: RwLock::new(None),
            selected_file: RwLock::new(None),
            selected_file_size: RwLock::new(None),
            sheet_count: RwLock::new(None),
            export_format: RwLock::new(ExportFormat::default()),
            theme_mode: RwLock::new(ThemeModeSetting::default()),
            effective_themes: RwLock::new(Vec::new()),
        }
    }

    pub fn with_workspace_id(workspace_id: EntityId) -> Self {
        let state = Self::new();
        state.register_workspace(workspace_id);
        state
    }

    pub fn register_workspace(&self, workspace_id: EntityId) {
        self.workspace_ids.write().insert(workspace_id);
    }

    pub fn unregister_workspace(&self, workspace_id: EntityId) {
        self.workspace_ids.write().remove(&workspace_id);
    }

    pub fn workspace_ids(&self) -> Vec<EntityId> {
        self.workspace_ids.read().iter().copied().collect()
    }

    pub fn notify_workspaces(cx: &mut App) {
        let workspace_ids = Self::global(cx).workspace_ids();
        for workspace_id in workspace_ids {
            cx.notify(workspace_id);
        }
    }

    pub fn apply_load_result(&self, result: LoadResult) {
        self.set_file_metadata(result.file_size, result.sheet_count);
        self.load_component(result.compo, result.directory, result.file);
    }

    /// Load component and related info atomically
    pub fn load_component(&self, compo: base::Component, dir: PathBuf, file: PathBuf) {
        *self.component.write() = Some(Arc::new(compo));
        *self.directory.write() = Some(Arc::new(dir));
        *self.selected_file.write() = Some(Arc::new(file));
    }

    /// Store metadata for the selected file.
    pub fn set_file_metadata(&self, file_size: Option<u64>, sheet_count: Option<usize>) {
        *self.selected_file_size.write() = file_size;
        *self.sheet_count.write() = sheet_count;
    }

    /// Check if a file is selected
    pub fn is_file_selected(&self) -> bool {
        self.selected_file.read().is_some()
    }

    /// Get the selected file path
    pub fn get_selected_file(&self) -> Option<Arc<PathBuf>> {
        self.selected_file.read().clone()
    }

    /// Get the selected file size in bytes
    pub fn get_selected_file_size(&self) -> Option<u64> {
        *self.selected_file_size.read()
    }

    /// Get the sheet count of the loaded workbook
    pub fn get_sheet_count(&self) -> Option<usize> {
        *self.sheet_count.read()
    }

    /// Get the selected export format.
    pub fn get_export_format(&self) -> ExportFormat {
        *self.export_format.read()
    }

    /// Set the export format.
    pub fn set_export_format(&self, format: ExportFormat) {
        *self.export_format.write() = format;
    }

    /// Get the current theme mode setting.
    pub fn get_theme_mode(&self) -> ThemeModeSetting {
        *self.theme_mode.read()
    }

    /// Set the current theme mode setting.
    pub fn set_theme_mode(&self, mode: ThemeModeSetting) {
        *self.theme_mode.write() = mode;
    }

    pub fn effective_themes(&self) -> Vec<ThemeConfig> {
        self.effective_themes.read().clone()
    }

    pub fn set_effective_themes(&self, themes: Vec<ThemeConfig>) {
        *self.effective_themes.write() = themes;
    }

    /// Get the directory path
    pub fn get_directory(&self) -> Option<Arc<PathBuf>> {
        self.directory.read().clone()
    }

    /// Get component for internal use (processing module)
    pub fn component(&self) -> Option<Arc<base::Component>> {
        self.component.read().clone()
    }

    /// Clear loaded data and selected file.
    pub fn clear_selection(&self) {
        *self.component.write() = None;
        *self.directory.write() = None;
        *self.selected_file.write() = None;
        *self.selected_file_size.write() = None;
        *self.sheet_count.write() = None;
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}
