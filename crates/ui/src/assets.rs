use gpui::AssetSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

#[derive(RustEmbed)]
#[folder = "../../assets/themes"]
#[include = "**/*"]
#[exclude = "*.DS_Store"]
pub struct ThemeAssets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        if let Some(f) = Self::get(path) {
            return Ok(Some(f.data));
        }
        gpui_component_assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        let mut set = std::collections::BTreeSet::<String>::new();

        for p in Self::iter() {
            if p.starts_with(path) {
                set.insert(p.to_string());
            }
        }

        for p in gpui_component_assets::Assets.list(path)? {
            set.insert(p.to_string());
        }

        Ok(set.into_iter().map(Into::into).collect())
    }
}
