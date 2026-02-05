use crate::processing::base;
use gpui::EntityId;
use parking_lot::RwLock;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ExportFormat {
    #[default]
    Ipxact,
    Regvue,
}

pub struct AppState {
    component: RwLock<Option<base::Component>>,
    directory: RwLock<Option<PathBuf>>,
    selected_file: RwLock<Option<PathBuf>>,
    selected_file_size: RwLock<Option<u64>>,
    sheet_count: RwLock<Option<usize>>,
    export_format: RwLock<ExportFormat>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component: RwLock::new(None),
            directory: RwLock::new(None),
            workspace_id: RwLock::new(None),
            selected_file: RwLock::new(None),
            selected_file_size: RwLock::new(None),
            sheet_count: RwLock::new(None),
            export_format: RwLock::new(ExportFormat::default()),
        }
    }

    /// Load component and related info atomically
    pub fn load_component(&self, compo: base::Component, dir: PathBuf, file: PathBuf) {
        *self.component.write() = Some(compo);
        *self.directory.write() = Some(dir);
        *self.selected_file.write() = Some(file);
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
    pub fn get_selected_file(&self) -> Option<PathBuf> {
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
    pub fn get_directory(&self) -> Option<PathBuf> {
        self.directory.read().clone()
    }

    /// Get component for internal use (processing module)
    /// Returns a cloned Option to avoid exposing the internal guard type
    #[doc(hidden)]
    pub fn component(&self) -> Option<base::Component> {
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

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
