use gpui::*;
use gpui_component::{
    IconName, Sizable as _, TitleBar, WindowExt as _,
    badge::Badge,
    button::{Button, ButtonVariants as _},
    menu::AppMenuBar,
};

pub struct AppTitleBar {}

impl AppTitleBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();
        let github = Button::new("github")
            .icon(IconName::GitHub)
            .small()
            .ghost()
            .on_click(|_, _, cx| cx.open_url("https://github.com/BeriBeli/irgen-gpui"));

        let bell = Badge::new().count(notifications_count).max(99).child(
            Button::new("bell")
                .small()
                .ghost()
                .compact()
                .icon(IconName::Bell),
        );

        TitleBar::new()
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(AppMenuBar::new(window, cx)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .child(github)
                    .child(bell),
            )
    }
}
