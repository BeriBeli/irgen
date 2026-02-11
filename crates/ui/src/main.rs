// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented
)]

mod assets;
mod config;
mod global;
mod app;

use assets::Assets;
use app::window::*;

use global::{GlobalState, ThemeModeSetting};
use gpui::*;
use gpui_component::{Theme, ThemeConfig, ThemeMode, ThemeRegistry};
use std::rc::Rc;
use app::workspace::Workspace;

fn refresh_effective_themes_in_global(cx: &mut App) {
    let themes = config::load_effective_themes_or_default();

    if cx.has_global::<GlobalState>() {
        GlobalState::global(cx).set_effective_themes(themes);
    } else {
        let state = GlobalState::new();
        state.set_effective_themes(themes);
        cx.set_global(state);
    }
}

fn resolve_theme_by_name(
    target_name: Option<&str>,
    target_mode: ThemeMode,
    preferred_themes: &[ThemeConfig],
    cx: &App,
) -> Option<Rc<ThemeConfig>> {
    let target_name = target_name?;

    preferred_themes
        .iter()
        .find(|theme| theme.name.as_ref() == target_name && theme.mode == target_mode)
        .cloned()
        .map(Rc::new)
        .or_else(|| {
            ThemeRegistry::global(cx)
                .themes()
                .values()
                .find(|theme| theme.name.as_ref() == target_name && theme.mode == target_mode)
                .cloned()
        })
}

fn restore_theme_from_config(cx: &mut App) {
    let app_config = config::load_app_config_or_default();
    let theme_prefs = app_config.theme;
    let preferred_themes = if cx.has_global::<GlobalState>() {
        GlobalState::global(cx).effective_themes()
    } else {
        config::load_effective_themes_or_default()
    };

    let light_theme = resolve_theme_by_name(
        theme_prefs.light.as_deref(),
        ThemeMode::Light,
        &preferred_themes,
        cx,
    );
    let dark_theme = resolve_theme_by_name(
        theme_prefs.dark.as_deref(),
        ThemeMode::Dark,
        &preferred_themes,
        cx,
    );

    {
        let theme = Theme::global_mut(cx);
        if let Some(light_theme) = light_theme {
            theme.light_theme = light_theme;
        }
        if let Some(dark_theme) = dark_theme {
            theme.dark_theme = dark_theme;
        }
    }

    match theme_prefs.mode {
        ThemeModeSetting::System => Theme::sync_system_appearance(None, cx),
        ThemeModeSetting::Light => Theme::change(ThemeMode::Light, None, cx),
        ThemeModeSetting::Dark => Theme::change(ThemeMode::Dark, None, cx),
    }

    if cx.has_global::<GlobalState>() {
        GlobalState::global(cx).set_theme_mode(theme_prefs.mode);
    } else {
        let state = GlobalState::new();
        state.set_theme_mode(theme_prefs.mode);
        state.set_effective_themes(preferred_themes);
        cx.set_global(state);
    }
}

fn main() {
    let application = Application::new().with_assets(Assets);

    application.run(|cx: &mut App| {
        gpui_component::init(cx);

        if let Err(err) = config::ensure_dirs() {
            eprintln!("Failed to initialize config directories: {}", err);
        }

        refresh_effective_themes_in_global(cx);
        restore_theme_from_config(cx);

        match config::themes_dir() {
            Ok(themes_dir) => {
                if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, |cx| {
                    refresh_effective_themes_in_global(cx);
                    restore_theme_from_config(cx);
                }) {
                    eprintln!("Failed to initialize theme watch directory: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Failed to resolve themes directory: {}", err);
            }
        }

        let window_options = get_window_options(cx);
        if let Err(err) = cx.open_window(window_options, |win, cx| {
            let workspace_view = Workspace::view(win, cx);
            cx.new(|cx| gpui_component::Root::new(workspace_view, win, cx))
        }) {
            eprintln!("Failed to open main window: {}", err);
        }
    });
}
