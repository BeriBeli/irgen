use gpui::*;
use gpui_component::TitleBar;

pub fn get_window_options(cx: &mut App) -> WindowOptions {
    let mut window_size = size(px(800.0), px(600.0));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        let max_width = display_size.width * 0.85;
        let max_height = display_size.height * 0.85;
        window_size.width = window_size.width.min(max_width);
        window_size.height = window_size.height.min(max_height);
    }
    let bounds = Bounds::centered(None, window_size, cx);
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        window_min_size: Some(window_size),
        titlebar: Some(TitleBar::title_bar_options()),
        ..Default::default()
    }
}
