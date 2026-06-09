mod anchor;
mod error;
mod html;
pub mod model;
mod view;

pub use error::Error;
pub use html::{
    HtmlPage, HtmlSite, serialize_html, serialize_html_site, serialize_html_site_stream,
};
