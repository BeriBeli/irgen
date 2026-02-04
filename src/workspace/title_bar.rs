use gpui::*;
use gpui::prelude::*;
use gpui_component::{
    ActiveTheme as _,
    IconName, Sizable as _, TitleBar, WindowExt as _,
    button::{Button, ButtonVariants as _},
    menu::AppMenuBar,
    white,
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

        let bell = {
            let bell_button = Button::new("bell")
                .small()
                .ghost()
                .compact()
                .icon(IconName::Bell);
            let count_label = if notifications_count > 99 {
                "99+".to_string()
            } else {
                notifications_count.to_string()
            };

            div()
                .relative()
                .child(bell_button)
                .when(notifications_count > 0, |this| {
                    this.child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .rounded_full()
                            .bg(cx.theme().red)
                            .text_color(white())
                            .border_1()
                            .border_color(white())
                            .px_1p5()
                            .min_w_3p5()
                            .text_xs()
                            .line_height(relative(1.))
                            .child(count_label),
                    )
                })
        };

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
                    .gap_3()
                    .pr_5()
                    .child(github)
                    .child(bell),
            )
    }
}
