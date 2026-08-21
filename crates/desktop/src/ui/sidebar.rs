use gpui::{
    Context, Entity, IntoElement, InteractiveElement, ParentElement, StatefulInteractiveElement,
    Styled, Window, div, px,
};
use woocraft::{
    ActiveTheme, Button, ButtonVariants as _, Icon, IconName, Selectable as _, StyledExt, h_flex,
    v_flex,
};

use crate::{
    i18n,
    models::ScopeData,
    ui::{Page, RootView},
};

/// Renders the navigation sidebar.
pub(crate) fn render_sidebar(
    _window: &mut Window,
    cx: &mut Context<RootView>,
    this: &Entity<RootView>,
    show_sidebar: bool,
    page: &Page,
    scopes: &[ScopeData],
    online: bool,
    api_port: u16,
) -> impl IntoElement {
    let weak = this.downgrade();
    let theme = cx.theme();
    let sidebar_width = px(256.);

    v_flex()
        .w(if show_sidebar { sidebar_width } else { px(0.) })
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .bg(theme.tab_bar)
        .border_r_1()
        .border_color(theme.border)
        .child(sidebar_header())
        .child(nav_item(
            "nav-home",
            IconName::Home,
            i18n::t("Get Started"),
            matches!(page, Page::Home),
            {
                let weak = weak.clone();
                move |_, _, cx| {
                    let _ = weak.update(cx, |root, cx| root.change_page(Page::Home, cx));
                }
            },
        ))
        .child(nav_item(
            "nav-logs",
            IconName::Code,
            i18n::t("Network logs"),
            matches!(page, Page::Logs),
            {
                let weak = weak.clone();
                move |_, _, cx| {
                    let _ = weak.update(cx, |root, cx| root.change_page(Page::Logs, cx));
                }
            },
        ))
        .child(
            div()
                .mt_2()
                .mb_2()
                .mx_4()
                .h_px()
                .bg(theme.border),
        )
        .child(nav_item(
            "nav-default-scope",
            IconName::GlobeStar,
            i18n::t("Default Scope"),
            matches!(page, Page::Scope(host) if host == "default-scope"),
            {
                let weak = weak.clone();
                move |_, _, cx| {
                    let _ = weak.update(cx, |root, cx| {
                        root.change_page(Page::Scope("default-scope".to_string()), cx)
                    });
                }
            },
        ))
        .children(scopes.iter().map(|scope| {
            let host = scope.host.clone();
            let active = matches!(page, Page::Scope(h) if *h == scope.host);
            let icon = if scope.state == "pending" {
                IconName::Warning
            } else {
                IconName::LockClosed
            };
            nav_item(
                format!("nav-scope-{host}"),
                icon,
                scope.name.clone(),
                active,
                {
                    let weak = weak.clone();
                    move |_, _, cx| {
                        let _ = weak.update(cx, |root, cx| {
                            root.change_page(Page::Scope(host.clone()), cx)
                        });
                    }
                },
            )
        }))
        .child(div().flex_1())
        .child(nav_item(
            "nav-settings",
            IconName::Settings,
            i18n::t("Settings"),
            matches!(page, Page::Settings),
            {
                let weak = weak.clone();
                move |_, _, cx| {
                    let _ = weak.update(cx, |root, cx| root.change_page(Page::Settings, cx));
                }
            },
        ))
        .child(controller_port_item(
            cx, online, api_port, weak.clone(),
        ))
}

fn sidebar_header() -> impl IntoElement {
    h_flex()
        .items_center()
        .gap_2()
        .px_4()
        .py_3()
        .child(Icon::new(IconName::GlobeStar).size(px(20.)))
        .child(
            div()
                .text_sm()
                .font_semibold()
                .child("WebSocket Reflector X"),
        )
}

/// A full-width flat navigation button with an active state.
fn nav_item(
    id: impl Into<gpui::ElementId>,
    icon: IconName,
    label: impl Into<gpui::SharedString>,
    active: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> Button {
    Button::new(id)
        .flat()
        .expand(true)
        .icon(Icon::new(icon))
        .label(label)
        .selected(active)
        .on_click(on_click)
}

/// Bottom row showing the controller port; copies the API address when online
/// and opens the network logs otherwise.
fn controller_port_item(
    cx: &mut Context<RootView>,
    online: bool,
    api_port: u16,
    weak: gpui::WeakEntity<RootView>,
) -> impl IntoElement {
    let theme = cx.theme();

    div()
        .id("controller-port")
        .mx_2()
        .mb_3()
        .px_3()
        .py_2()
        .flex()
        .items_center()
        .gap_2()
        .rounded_md()
        .hover(|this| this.bg(theme.secondary_hover.opacity(0.6)))
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            if online {
                let address = format!("http://127.0.0.1:{api_port}");
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(address));
            } else {
                let _ = weak.update(cx, |root, cx| root.change_page(Page::Logs, cx));
            }
        })
        .child(
            h_flex()
                .flex_1()
                .items_center()
                .gap_2()
                .child(
                    Icon::new(if online {
                        IconName::FlashFlow
                    } else {
                        IconName::GlobeWarning
                    })
                    .size(px(16.))
                    .text_color(if online {
                        theme.success
                    } else {
                        theme.danger
                    }),
                )
                .child(
                    div()
                        .flex_1()
                        .text_sm()
                        .child(i18n::t("Controller port")),
                )
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.primary)
                        .child(if online {
                            api_port.to_string()
                        } else {
                            "--".to_string()
                        }),
                ),
        )
}
