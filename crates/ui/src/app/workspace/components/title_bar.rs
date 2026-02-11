use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, Sizable as _, StyledExt as _, Theme,
    ThemeConfig, ThemeMode, ThemeRegistry, TitleBar, WindowExt as _,
    button::{Button, ButtonVariants as _},
    menu::{AppMenuBar, DropdownMenu as _, PopupMenuItem},
    notification::NotificationType,
    popover::Popover,
    scroll::ScrollableElement as _,
    white,
};
use std::collections::HashMap;
use std::rc::Rc;

use crate::config;
use crate::global::{GlobalState, ThemeModeSetting};

pub struct WorkspaceTitleBar {}

fn merge_themes_with_overrides(
    mut base_themes: Vec<Rc<ThemeConfig>>,
    override_themes: &[ThemeConfig],
) -> Vec<Rc<ThemeConfig>> {
    let mut override_themes_by_key = override_themes
        .iter()
        .cloned()
        .map(|theme| ((theme.name.to_string(), theme.mode), Rc::new(theme)))
        .collect::<HashMap<_, _>>();

    for theme in &mut base_themes {
        if let Some(override_theme) =
            override_themes_by_key.remove(&(theme.name.to_string(), theme.mode))
        {
            *theme = override_theme;
        }
    }

    base_themes.extend(override_themes_by_key.into_values());
    base_themes.sort_by(|a, b| {
        b.is_default
            .cmp(&a.is_default)
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            .then(a.mode.name().cmp(b.mode.name()))
    });

    base_themes
}

fn notification_meta(
    notification_type: NotificationType,
    cx: &App,
) -> (IconName, Hsla, &'static str) {
    match notification_type {
        NotificationType::Info => (IconName::Info, cx.theme().info, "Info"),
        NotificationType::Success => (IconName::CircleCheck, cx.theme().success, "Success"),
        NotificationType::Warning => (IconName::TriangleAlert, cx.theme().warning, "Warning"),
        NotificationType::Error => (IconName::CircleX, cx.theme().danger, "Error"),
    }
}

impl WorkspaceTitleBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let persist_theme_preferences = |cx: &mut App| {
            let theme = Theme::global(cx);
            let theme_prefs = config::ThemePrefs {
                mode: GlobalState::global(cx).get_theme_mode(),
                light: Some(theme.light_theme.name.to_string()),
                dark: Some(theme.dark_theme.name.to_string()),
            };

            cx.spawn(async move |cx| {
                let result = cx
                    .background_spawn(async move { config::save_theme_preferences(theme_prefs) })
                    .await;
                if let Err(err) = result {
                    eprintln!("Failed to save app config: {}", err);
                }
            })
            .detach();
        };

        let state = GlobalState::global(cx);
        let theme_mode = state.get_theme_mode();
        let theme_mode_icon = match theme_mode {
            ThemeModeSetting::System => IconName::Settings2,
            ThemeModeSetting::Light => IconName::Sun,
            ThemeModeSetting::Dark => IconName::Moon,
        };
        let active_theme_name = Theme::global(cx).theme_name().to_string();
        let active_theme_mode = Theme::global(cx).mode;
        let override_themes = state.effective_themes();
        let base_themes = ThemeRegistry::global(cx)
            .sorted_themes()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let unread_notifications = state.unread_notification_count();
        let theme_mode_picker = Button::new("title-theme-mode-toggle")
            .icon(theme_mode_icon)
            .small()
            .ghost()
            .compact()
            .on_click(move |_, window, cx| {
                let next_mode = match GlobalState::global(cx).get_theme_mode() {
                    ThemeModeSetting::System => ThemeModeSetting::Light,
                    ThemeModeSetting::Light => ThemeModeSetting::Dark,
                    ThemeModeSetting::Dark => ThemeModeSetting::System,
                };

                GlobalState::global(cx).set_theme_mode(next_mode);
                match next_mode {
                    ThemeModeSetting::System => Theme::sync_system_appearance(Some(window), cx),
                    ThemeModeSetting::Light => Theme::change(ThemeMode::Light, Some(window), cx),
                    ThemeModeSetting::Dark => Theme::change(ThemeMode::Dark, Some(window), cx),
                }
                persist_theme_preferences(cx);

                GlobalState::notify_workspaces(cx);
            });

        let theme_preset_picker = Button::new("title-theme-preset")
            .icon(IconName::Palette)
            .compact()
            .small()
            .ghost()
            .dropdown_menu({
                let active_theme_name = active_theme_name.clone();
                let base_themes = base_themes.clone();
                let override_themes = override_themes.clone();
                move |menu, _, _cx| {
                    let themes = merge_themes_with_overrides(base_themes.clone(), &override_themes);
                    let menu = menu
                        .max_h(px(360.))
                        .scrollable(true)
                        .item(PopupMenuItem::label("Theme Preset"))
                        .item(PopupMenuItem::separator());

                    themes.iter().fold(menu, |menu, theme| {
                        let theme_name = theme.name.to_string();
                        let target_mode = theme.mode;
                        let checked =
                            theme_name == active_theme_name && target_mode == active_theme_mode;
                        let item_label = format!("{} ({})", theme_name, target_mode.name());
                        let theme = theme.clone();

                        menu.item(PopupMenuItem::new(item_label).checked(checked).on_click(
                            move |_, window, cx| {
                                {
                                    let active_theme = Theme::global_mut(cx);
                                    if target_mode.is_dark() {
                                        active_theme.dark_theme = theme.clone();
                                    } else {
                                        active_theme.light_theme = theme.clone();
                                    }
                                }

                                let mode_setting = if target_mode.is_dark() {
                                    ThemeModeSetting::Dark
                                } else {
                                    ThemeModeSetting::Light
                                };
                                GlobalState::global(cx).set_theme_mode(mode_setting);
                                Theme::change(target_mode, Some(window), cx);
                                persist_theme_preferences(cx);
                                GlobalState::notify_workspaces(cx);
                            },
                        ))
                    })
                }
            });

        let github = Button::new("github")
            .icon(IconName::GitHub)
            .small()
            .ghost()
            .on_click(|_, _, cx| cx.open_url("https://github.com/BeriBeli/irgen"));

        let bell = {
            let count_label = if unread_notifications > 99 {
                "99+".to_string()
            } else {
                unread_notifications.to_string()
            };

            div()
                .relative()
                .child(
                    Popover::new("notification-center-popover")
                        .anchor(Corner::TopRight)
                        .on_open_change(|open, _, cx: &mut App| {
                            if *open {
                                GlobalState::global(cx).mark_notifications_read();
                                GlobalState::notify_workspaces(cx);
                            }
                        })
                        .trigger(
                            Button::new("bell")
                                .small()
                                .ghost()
                                .compact()
                                .icon(IconName::Bell),
                        )
                        .content(|_, _window, cx| {
                            let state = GlobalState::global(cx);
                            let notification_history = state.notification_history();
                            let history_len = notification_history.len();
                            let has_notifications = history_len > 0;
                            let notification_content = if has_notifications {
                                let items = notification_history
                                    .iter()
                                    .rev()
                                    .enumerate()
                                    .map(|(idx, note)| {
                                        let (icon_name, icon_color, type_label) =
                                            notification_meta(note.type_, cx);

                                        div()
                                            .id(("notification-item", idx))
                                            .w_full()
                                            .flex()
                                            .items_start()
                                            .gap_3()
                                            .px_3()
                                            .py_2()
                                            .border_1()
                                            .border_color(cx.theme().border)
                                            .bg(cx.theme().background)
                                            .rounded(cx.theme().radius)
                                            .child(
                                                Icon::new(icon_name)
                                                    .text_color(icon_color)
                                                    .mt(px(2.)),
                                            )
                                            .child(
                                                div()
                                                    .min_w_0()
                                                    .flex_1()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_medium()
                                                            .text_color(cx.theme().muted_foreground)
                                                            .child(type_label),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .line_height(relative(1.3))
                                                            .child(note.message.clone()),
                                                    ),
                                            )
                                    })
                                    .collect::<Vec<_>>();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .children(items)
                                    .into_any_element()
                            } else {
                                div()
                                    .h(px(140.))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(
                                        Icon::new(IconName::Bell)
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(div().text_sm().child("No notifications yet"))
                                    .into_any_element()
                            };

                            div()
                                .id("notification-center")
                                .w(px(440.))
                                .max_h(px(420.))
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_semibold()
                                                .child(format!("Notifications ({history_len})")),
                                        )
                                        .child(
                                            Button::new("notification-clear")
                                                .xsmall()
                                                .ghost()
                                                .label("Clear")
                                                .disabled(!has_notifications)
                                                .on_click(|_, window, cx| {
                                                    GlobalState::global(cx)
                                                        .clear_notification_history();
                                                    window.clear_notifications(cx);
                                                    GlobalState::notify_workspaces(cx);
                                                }),
                                        ),
                                )
                                .child(
                                    div()
                                        .max_h(px(320.))
                                        .overflow_y_scrollbar()
                                        .pr_1()
                                        .child(notification_content),
                                )
                        }),
                )
                .when(unread_notifications > 0, |this| {
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
                    .child(theme_mode_picker)
                    .child(theme_preset_picker)
                    .child(github)
                    .child(bell),
            )
    }
}
