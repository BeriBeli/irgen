use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "templates/**/*"]
pub struct TemplateAssets;
