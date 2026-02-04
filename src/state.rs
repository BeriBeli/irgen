use crate::services::base;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AppState {
    pub component: RwLock<Option<base::Component>>,
    pub directory: RwLock<Option<PathBuf>>,
    pub is_selected: AtomicBool,
    pub selected_file: RwLock<Option<PathBuf>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component: RwLock::new(None),
            directory: RwLock::new(None),
            is_selected: AtomicBool::new(false),
            selected_file: RwLock::new(None),
        }
    }

    /// 原子性地加载组件及相关信息
    pub fn load_component(
        &self,
        compo: base::Component,
        dir: PathBuf,
        file: PathBuf,
    ) {
        *self.component.write() = Some(compo);
        *self.directory.write() = Some(dir);
        *self.selected_file.write() = Some(file);
        self.is_selected.store(true, Ordering::Release);
    }

    /// 获取是否已选择文件
    pub fn is_file_selected(&self) -> bool {
        self.is_selected.load(Ordering::Acquire)
    }

    /// 获取选择的文件路径
    pub fn get_selected_file(&self) -> Option<PathBuf> {
        self.selected_file.read().clone()
    }

    /// 获取目录路径
    pub fn get_directory(&self) -> Option<PathBuf> {
        self.directory.read().clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
