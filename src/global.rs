use crate::processing::{LoadResult, base};
use gpui::{EntityId, Global};
use parking_lot::RwLock;
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

pub struct GlobalState {
    workspace_id: RwLock<Option<EntityId>>,
    component: RwLock<Option<Arc<base::Component>>>,
    directory: RwLock<Option<Arc<PathBuf>>>,
    selected_file: RwLock<Option<Arc<PathBuf>>>,
    selected_file_size: RwLock<Option<u64>>,
    sheet_count: RwLock<Option<usize>>,
    export_format: RwLock<ExportFormat>,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            workspace_id: RwLock::new(None),
            component: RwLock::new(None),
            directory: RwLock::new(None),
            selected_file: RwLock::new(None),
            selected_file_size: RwLock::new(None),
            sheet_count: RwLock::new(None),
            export_format: RwLock::new(ExportFormat::default()),
        }
    }

    pub fn with_workspace_id(workspace_id: EntityId) -> Self {
        let state = Self::new();
        state.set_workspace_id(workspace_id);
        state
    }

    pub fn set_workspace_id(&self, workspace_id: EntityId) {
        *self.workspace_id.write() = Some(workspace_id);
    }

    pub fn workspace_id(&self) -> Option<EntityId> {
        self.workspace_id.read().as_ref().copied()
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
