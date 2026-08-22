use gpui::{Context, Entity, IntoElement, ParentElement, Styled, Window, div, px};
use woocraft::{
    ActiveTheme, Button, ButtonVariants as _, Icon, IconName, Selectable as _, Theme, v_flex,
};

use crate::{
    i18n,
    models::ScopeData,
    ui::{Page, RootView},
};

/// Renders the navigation sidebar.
///
/// The whole sidebar is a `p-1 gap-1` flex column; every entry (including the
/// controller port row at the bottom) is the same full-width flat [`Button`]
/// component.
pub(crate) fn render_sidebar(
    _window: &mut Window, cx: &mut Context<RootView>, this: &Entity<RootView>, page: &Page,
    scopes: &[ScopeData], online: bool, api_port: u16,
) -> impl IntoElement {
    let weak = this.downgrade();
    let theme = cx.theme();

    v_flex()
        .w(px(256.))
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .p_1()
        .gap_1()
        .bg(theme.tab_bar)
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
        .child(div().mx_1().h_px().bg(theme.border))
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
        .child(controller_port_item(online, api_port, weak.clone(), theme))
}

/// A full-width flat navigation button with an active state.
fn nav_item(
    id: impl Into<gpui::ElementId>, icon: IconName, label: impl Into<gpui::SharedString>,
    active: bool, on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> Button {
    Button::new(id)
        .flat()
        .expand(true)
        .icon(Icon::new(icon))
        .label(label)
        .selected(active)
        .on_click(on_click)
}

/// The controller port entry at the bottom of the sidebar. Uses the same
/// [`nav_item`] button as every other entry; clicking it copies the API
/// address when online and opens the network logs otherwise. The port number
/// is right-aligned via an extra flex child.
fn controller_port_item(
    online: bool, api_port: u16, weak: gpui::WeakEntity<RootView>, theme: &Theme,
) -> Button {
    nav_item(
        "nav-controller-port",
        if online {
            IconName::FlashFlow
        } else {
            IconName::GlobeWarning
        },
        i18n::t("Controller port"),
        false,
        move |_, _, cx| {
            if online {
                let address = format!("http://127.0.0.1:{api_port}");
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(address));
            } else {
                let _ = weak.update(cx, |root, cx| root.change_page(Page::Logs, cx));
            }
        },
    )
    .child(
        div()
            .flex_1()
            .flex_shrink_0()
            .text_right()
            .text_color(theme.success)
            .child(if online {
                api_port.to_string()
            } else {
                "--".to_string()
            }),
    )
}
