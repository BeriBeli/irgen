use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, IconName, Sizable as _, Theme, ThemeConfig, ThemeMode, ThemeRegistry,
    TitleBar, WindowExt as _,
    button::{Button, ButtonVariants as _},
    menu::{AppMenuBar, DropdownMenu as _, PopupMenuItem},
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

        let notifications_count = window.notifications(cx).len();
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
                    .child(theme_mode_picker)
                    .child(theme_preset_picker)
                    .child(github)
                    .child(bell),
            )
    }
}
