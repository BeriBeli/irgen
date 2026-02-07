use std::fs;
use std::path::PathBuf;

use crate::assets::ThemeAssets;
use crate::error::Error;
use crate::global::ThemeModeSetting;
use gpui_component::{ThemeConfig, ThemeMode, ThemeSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub theme: ThemePrefs,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemePrefs {
    pub mode: ThemeModeSetting,
    pub light: Option<String>,
    pub dark: Option<String>,
}

pub fn config_root() -> Result<PathBuf, Error> {
    dirs::home_dir()
        .map(|path| path.join(".config").join("irgen"))
        .ok_or_else(|| Error::ConfigInitialization {
            message: "Failed to resolve home directory for user config.".into(),
        })
}

pub fn themes_dir() -> Result<PathBuf, Error> {
    Ok(config_root()?.join("themes"))
}

pub fn templates_dir() -> Result<PathBuf, Error> {
    Ok(config_root()?.join("templates"))
}

pub fn config_file_path() -> Result<PathBuf, Error> {
    Ok(config_root()?.join("config.json"))
}

pub fn ensure_dirs() -> Result<(), Error> {
    let root = config_root()?;
    fs::create_dir_all(&root)?;
    fs::create_dir_all(root.join("themes"))?;
    fs::create_dir_all(root.join("templates"))?;
    Ok(())
}

fn sort_themes(mut themes: Vec<ThemeConfig>) -> Vec<ThemeConfig> {
    themes.sort_by(|a, b| {
        b.is_default
            .cmp(&a.is_default)
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            .then(a.mode.name().cmp(b.mode.name()))
    });
    themes
}

fn theme_key(theme: &ThemeConfig) -> (String, ThemeMode) {
    (theme.name.to_string(), theme.mode)
}

pub fn merge_theme_lists(base: Vec<ThemeConfig>, overrides: Vec<ThemeConfig>) -> Vec<ThemeConfig> {
    let mut themes = std::collections::HashMap::<(String, ThemeMode), ThemeConfig>::new();
    for theme in base {
        themes.insert(theme_key(&theme), theme);
    }
    for theme in overrides {
        themes.insert(theme_key(&theme), theme);
    }
    sort_themes(themes.into_values().collect())
}

fn list_user_theme_files() -> Result<Vec<PathBuf>, Error> {
    let themes_dir = themes_dir()?;
    if !themes_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = fs::read_dir(&themes_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let is_json = path.extension().and_then(|s| s.to_str()) == Some("json");
            (path.is_file() && is_json).then_some(path)
        })
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

pub fn load_asset_themes() -> Result<Vec<ThemeConfig>, Error> {
    let mut themes = std::collections::HashMap::<(String, ThemeMode), ThemeConfig>::new();

    for asset_path in ThemeAssets::iter() {
        let asset_path: &str = asset_path.as_ref();
        if !asset_path.ends_with(".json") {
            continue;
        }

        let Some(embedded) = ThemeAssets::get(asset_path) else {
            eprintln!("Embedded theme asset missing: {}", asset_path);
            continue;
        };

        let content = match std::str::from_utf8(embedded.data.as_ref()) {
            Ok(content) => content,
            Err(err) => {
                eprintln!(
                    "Ignored invalid UTF-8 embedded theme {}: {}",
                    asset_path, err
                );
                continue;
            }
        };

        match serde_json::from_str::<ThemeSet>(content) {
            Ok(theme_set) => {
                for theme in theme_set.themes {
                    themes.insert(theme_key(&theme), theme);
                }
            }
            Err(err) => {
                eprintln!("Ignored invalid embedded theme {}: {}", asset_path, err);
            }
        }
    }

    Ok(sort_themes(themes.into_values().collect()))
}

pub fn load_user_themes() -> Result<Vec<ThemeConfig>, Error> {
    let mut themes = std::collections::HashMap::<(String, ThemeMode), ThemeConfig>::new();

    for path in list_user_theme_files()? {
        let content = fs::read_to_string(&path)?;
        match serde_json::from_str::<ThemeSet>(&content) {
            Ok(theme_set) => {
                for theme in theme_set.themes {
                    themes.insert(theme_key(&theme), theme);
                }
            }
            Err(err) => {
                eprintln!("Ignored invalid theme file {}: {}", path.display(), err);
            }
        }
    }

    Ok(sort_themes(themes.into_values().collect()))
}

pub fn load_effective_themes() -> Result<Vec<ThemeConfig>, Error> {
    Ok(merge_theme_lists(load_asset_themes()?, load_user_themes()?))
}

pub fn load_effective_themes_or_default() -> Vec<ThemeConfig> {
    match load_effective_themes() {
        Ok(themes) => themes,
        Err(err) => {
            eprintln!("Failed to load effective themes: {}", err);
            Vec::new()
        }
    }
}

pub fn load_app_config() -> Result<AppConfig, Error> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let bytes = fs::read(path)?;
    let config = serde_json::from_slice::<AppConfig>(&bytes)?;
    Ok(config)
}

pub fn load_app_config_or_default() -> AppConfig {
    match load_app_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load app config, fallback to defaults: {}", err);
            AppConfig::default()
        }
    }
}

pub fn save_app_config(config: &AppConfig) -> Result<(), Error> {
    ensure_dirs()?;
    let path = config_file_path()?;
    let tmp_path = path.with_extension("json.tmp");
    let content = serde_json::to_vec_pretty(config)?;

    fs::write(&tmp_path, content)?;
    if let Err(err) = fs::rename(&tmp_path, &path) {
        if path.exists() {
            fs::remove_file(&path)?;
            fs::rename(&tmp_path, &path)?;
        } else {
            return Err(err.into());
        }
    }

    Ok(())
}

pub fn save_theme_preferences(theme: ThemePrefs) -> Result<(), Error> {
    save_app_config(&AppConfig { theme })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_theme(name: &str, mode: ThemeMode, is_default: bool) -> ThemeConfig {
        ThemeConfig {
            name: name.to_string().into(),
            mode,
            is_default,
            ..ThemeConfig::default()
        }
    }

    #[test]
    fn merges_asset_and_user_themes_with_user_priority() {
        let merged = merge_theme_lists(
            vec![make_theme("foo", ThemeMode::Light, true)],
            vec![make_theme("foo", ThemeMode::Light, false)],
        );

        assert_eq!(merged.len(), 1);
        assert_eq!(
            merged
                .iter()
                .find(|theme| theme.name.as_ref() == "foo" && theme.mode == ThemeMode::Light)
                .map(|theme| theme.is_default),
            Some(false)
        );
    }

    #[test]
    fn does_not_override_different_mode_with_same_name() {
        let merged = merge_theme_lists(
            vec![make_theme("foo", ThemeMode::Dark, true)],
            vec![make_theme("foo", ThemeMode::Light, false)],
        );

        assert_eq!(merged.len(), 2);
        assert_eq!(
            merged
                .iter()
                .filter(|theme| theme.name.as_ref() == "foo")
                .count(),
            2
        );
        assert!(merged.iter().any(|theme| {
            theme.name.as_ref() == "foo" && theme.mode == ThemeMode::Light && !theme.is_default
        }));
        assert!(
            merged
                .iter()
                .any(|theme| theme.name.as_ref() == "foo" && theme.mode == ThemeMode::Dark)
        );
    }

    #[test]
    fn effective_theme_sort_is_stable() {
        let merged = merge_theme_lists(
            vec![
                make_theme("beta", ThemeMode::Light, false),
                make_theme("zeta", ThemeMode::Dark, true),
                make_theme("alpha", ThemeMode::Light, true),
            ],
            Vec::new(),
        );

        let ordered = merged
            .iter()
            .map(|theme| (theme.name.to_string(), theme.mode, theme.is_default))
            .collect::<Vec<_>>();
        assert_eq!(
            ordered,
            vec![
                ("alpha".to_string(), ThemeMode::Light, true),
                ("zeta".to_string(), ThemeMode::Dark, true),
                ("beta".to_string(), ThemeMode::Light, false),
            ]
        );
    }
}
