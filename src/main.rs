// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod error;
mod global;
mod processing;
mod ui;

use assets::Assets;
use ui::window::*;

use gpui::*;
use ui::workspace::Workspace;

fn main() {
    let application = Application::new().with_assets(Assets);

    application.run(|cx: &mut App| {
        let window_options = get_window_options(cx);
        if let Err(err) = cx.open_window(window_options, |win, cx| {
            gpui_component::init(cx);
            let workspace_view = Workspace::view(win, cx);
            cx.new(|cx| gpui_component::Root::new(workspace_view, win, cx))
        }) {
            eprintln!("Failed to open main window: {}", err);
            return;
        }
    });
}
