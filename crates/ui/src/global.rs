use gpui::{App, EntityId, Global, ReadGlobal, SharedString};
use gpui_component::{ThemeConfig, notification::NotificationType};
use irgen_core::processing::{LoadResult, base};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
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

const NOTIFICATION_HISTORY_LIMIT: usize = 200;

#[derive(Debug, Clone)]
pub struct NotificationEntry {
    pub type_: NotificationType,
    pub message: SharedString,
}

pub struct GlobalState {
    workspace_ids: HashSet<EntityId>,
    component: Option<Arc<base::Component>>,
    directory: Option<Arc<PathBuf>>,
    selected_file: Option<Arc<PathBuf>>,
    selected_file_size: Option<u64>,
    sheet_count: Option<usize>,
    export_format: ExportFormat,
    theme_mode: ThemeModeSetting,
    effective_themes: Vec<ThemeConfig>,
    notification_history: VecDeque<NotificationEntry>,
    unread_notification_count: usize,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            workspace_ids: HashSet::new(),
            component: None,
            directory: None,
            selected_file: None,
            selected_file_size: None,
            sheet_count: None,
            export_format: ExportFormat::default(),
            theme_mode: ThemeModeSetting::default(),
            effective_themes: Vec::new(),
            notification_history: VecDeque::new(),
            unread_notification_count: 0,
        }
    }

    pub fn with_workspace_id(workspace_id: EntityId) -> Self {
        let state = Self::new();
        let mut state = state;
        state.register_workspace_inner(workspace_id);
        state
    }

    pub fn register_workspace(cx: &mut App, workspace_id: EntityId) {
        cx.global_mut::<Self>()
            .register_workspace_inner(workspace_id);
    }

    fn register_workspace_inner(&mut self, workspace_id: EntityId) {
        self.workspace_ids.insert(workspace_id);
    }

    pub fn unregister_workspace(cx: &mut App, workspace_id: EntityId) {
        cx.global_mut::<Self>()
            .unregister_workspace_inner(workspace_id);
    }

    fn unregister_workspace_inner(&mut self, workspace_id: EntityId) {
        self.workspace_ids.remove(&workspace_id);
    }

    pub fn workspace_ids(&self) -> Vec<EntityId> {
        self.workspace_ids.iter().copied().collect()
    }

    pub fn notify_workspaces(cx: &mut App) {
        let workspace_ids = Self::global(cx).workspace_ids();
        for workspace_id in workspace_ids {
            cx.notify(workspace_id);
        }
    }

    pub fn apply_load_result(cx: &mut App, result: LoadResult) {
        let state = cx.global_mut::<Self>();
        state.set_file_metadata(result.file_size, result.sheet_count);
        state.load_component(result.compo, result.directory, result.file);
    }

    /// Load component and related info atomically
    fn load_component(&mut self, compo: base::Component, dir: PathBuf, file: PathBuf) {
        self.component = Some(Arc::new(compo));
        self.directory = Some(Arc::new(dir));
        self.selected_file = Some(Arc::new(file));
    }

    /// Store metadata for the selected file.
    fn set_file_metadata(&mut self, file_size: Option<u64>, sheet_count: Option<usize>) {
        self.selected_file_size = file_size;
        self.sheet_count = sheet_count;
    }

    /// Check if a file is selected
    pub fn is_file_selected(&self) -> bool {
        self.selected_file.is_some()
    }

    /// Get the selected file path
    pub fn get_selected_file(&self) -> Option<Arc<PathBuf>> {
        self.selected_file.clone()
    }

    /// Get the selected file size in bytes
    pub fn get_selected_file_size(&self) -> Option<u64> {
        self.selected_file_size
    }

    /// Get the sheet count of the loaded workbook
    pub fn get_sheet_count(&self) -> Option<usize> {
        self.sheet_count
    }

    /// Get the selected export format.
    pub fn get_export_format(&self) -> ExportFormat {
        self.export_format
    }

    /// Set the export format.
    pub fn set_export_format(cx: &mut App, format: ExportFormat) {
        cx.global_mut::<Self>().export_format = format;
    }

    /// Get the current theme mode setting.
    pub fn get_theme_mode(&self) -> ThemeModeSetting {
        self.theme_mode
    }

    /// Set the current theme mode setting.
    pub fn set_theme_mode(cx: &mut App, mode: ThemeModeSetting) {
        cx.global_mut::<Self>().set_theme_mode_inner(mode);
    }

    fn set_theme_mode_inner(&mut self, mode: ThemeModeSetting) {
        self.theme_mode = mode;
    }

    pub fn effective_themes(&self) -> Vec<ThemeConfig> {
        self.effective_themes.clone()
    }

    pub fn set_effective_themes(cx: &mut App, themes: Vec<ThemeConfig>) {
        cx.global_mut::<Self>().set_effective_themes_inner(themes);
    }

    fn set_effective_themes_inner(&mut self, themes: Vec<ThemeConfig>) {
        self.effective_themes = themes;
    }

    pub fn push_notification(
        cx: &mut App,
        notification_type: NotificationType,
        message: impl Into<SharedString>,
    ) {
        let state = cx.global_mut::<Self>();
        state.notification_history.push_back(NotificationEntry {
            type_: notification_type,
            message: message.into(),
        });
        if state.notification_history.len() > NOTIFICATION_HISTORY_LIMIT {
            state.notification_history.pop_front();
        }

        let history_len = state.notification_history.len();
        state.unread_notification_count =
            state.unread_notification_count.saturating_add(1).min(history_len);
    }

    pub fn notification_history(&self) -> Vec<NotificationEntry> {
        self.notification_history.iter().cloned().collect()
    }

    pub fn unread_notification_count(&self) -> usize {
        self.unread_notification_count
    }

    pub fn mark_notifications_read(cx: &mut App) {
        cx.global_mut::<Self>().unread_notification_count = 0;
    }

    pub fn clear_notification_history(cx: &mut App) {
        let state = cx.global_mut::<Self>();
        state.notification_history.clear();
        state.unread_notification_count = 0;
    }

    /// Get the directory path
    pub fn get_directory(&self) -> Option<Arc<PathBuf>> {
        self.directory.clone()
    }

    /// Get component for internal use (processing module)
    pub fn component(&self) -> Option<Arc<base::Component>> {
        self.component.clone()
    }

    /// Clear loaded data and selected file.
    pub fn clear_selection(cx: &mut App) {
        let state = cx.global_mut::<Self>();
        state.component = None;
        state.directory = None;
        state.selected_file = None;
        state.selected_file_size = None;
        state.sheet_count = None;
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}
