use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};
use woocraft::{
    ActiveTheme, Button, ButtonVariants as _, Disableable as _, Icon, IconName,
    ScrollableElement as _, Sizable as _, h_flex, v_flex,
};

use crate::{
    daemon, i18n,
    models::{InstanceData, ScopeData},
    ui::{Page, RootView},
};

/// Renders the connections page for a scope.
pub(crate) fn render_connections(
    _window: &mut Window, cx: &mut Context<RootView>, root: &mut RootView, host: &str,
) -> impl IntoElement {
    let theme = cx.theme();
    let weak = cx.entity().downgrade();
    let state = root.state().clone();

    let scope = root.scope_for_page();
    let instances = root.scoped_instances();
    let is_default = host == "default-scope";
    let allowed = scope
        .as_ref()
        .map(|s| s.state == "allowed")
        .unwrap_or(false);

    v_flex()
        .gap_1()
        .px_8()
        .py_4()
        .overflow_y_scrollbar()
        .size_full()
        .child(
            // Header
            v_flex()
                .gap_1()
                .child(
                    h_flex()
                        .items_center()
                        .gap_1()
                        .child(
                            Icon::new(if is_default {
                                IconName::GlobeStar
                            } else if !allowed {
                                IconName::GlobeWarning
                            } else {
                                IconName::GlobeSync
                            })
                            .size(px(28.))
                            .text_color(theme.foreground),
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .child(
                                    div()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .child(scope_name(&scope, is_default)),
                                )
                                .child(div().text_color(theme.muted_foreground).child(
                                    if is_default {
                                        i18n::t("This is the default scope.").to_string()
                                    } else {
                                        scope.as_ref().map(|s| s.host.clone()).unwrap_or_default()
                                    },
                                )),
                        )
                        .child(
                            v_flex()
                                .items_end()
                                .child(
                                    div()
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .text_color(if is_default {
                                            theme.primary
                                        } else {
                                            theme.success
                                        })
                                        .child(if is_default {
                                            i18n::t("Manually Controlled")
                                        } else {
                                            i18n::t("External Controlled")
                                        }),
                                )
                                .child(div().text_color(theme.muted_foreground).child(
                                    if is_default {
                                        "basic".to_string()
                                    } else {
                                        scope
                                            .as_ref()
                                            .map(|s| s.features.to_string())
                                            .unwrap_or_default()
                                    },
                                )),
                        ),
                )
                .child(div().h_px().bg(theme.border))
                .child(
                    h_flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .flex_1()
                                .text_color(if is_default {
                                    theme.primary
                                } else if allowed {
                                    theme.success
                                } else {
                                    theme.danger
                                })
                                .child(if is_default {
                                    i18n::t("This scope is controlled by you manually.")
                                } else if allowed {
                                    i18n::t("This domain can control wsrx on your local network.")
                                } else {
                                    i18n::t("This domain requested controlling wsrx.")
                                }),
                        )
                        .when(is_default, |this| {
                            this.child(
                                Button::new("enjoy")
                                    .flat()
                                    .icon(Icon::new(IconName::CheckmarkCircle))
                                    .label(i18n::t("No Operation"))
                                    .disabled(true),
                            )
                        })
                        .when(!is_default && !allowed, |this| {
                            this.child(
                                Button::new("accept")
                                    .flat()
                                    .icon(Icon::new(IconName::CheckmarkCircle))
                                    .label(i18n::t("Accept"))
                                    .on_click({
                                        let state = state.clone();
                                        let host = host.to_string();
                                        move |_, _, _| {
                                            let state = state.clone();
                                            let host = host.clone();
                                            daemon::tokio_handle().spawn(async move {
                                                daemon::allow_scope(&state, &host).await;
                                            });
                                        }
                                    }),
                            )
                            .child(
                                Button::new("decline")
                                    .flat()
                                    .icon(Icon::new(IconName::DismissCircle))
                                    .label(i18n::t("Decline"))
                                    .on_click({
                                        let weak = weak.clone();
                                        let state = state.clone();
                                        let host = host.to_string();
                                        move |_, _, cx| {
                                            let state = state.clone();
                                            let host = host.clone();
                                            daemon::tokio_handle().spawn(async move {
                                                daemon::remove_scope(&state, &host).await;
                                            });
                                            let _ = weak.update(cx, |root, cx| {
                                                root.change_page(
                                                    Page::Scope("default-scope".to_string()),
                                                    cx,
                                                );
                                            });
                                        }
                                    }),
                            )
                        })
                        .when(!is_default && allowed, |this| {
                            this.child(
                                Button::new("remove")
                                    .flat()
                                    .icon(Icon::new(IconName::DismissCircle))
                                    .label(i18n::t("Remove"))
                                    .on_click({
                                        let weak = weak.clone();
                                        let state = state.clone();
                                        let host = host.to_string();
                                        move |_, _, cx| {
                                            let state = state.clone();
                                            let host = host.clone();
                                            daemon::tokio_handle().spawn(async move {
                                                daemon::remove_scope(&state, &host).await;
                                            });
                                            let _ = weak.update(cx, |root, cx| {
                                                root.change_page(
                                                    Page::Scope("default-scope".to_string()),
                                                    cx,
                                                );
                                            });
                                        }
                                    }),
                            )
                        }),
                )
                .child(div().h_px().bg(theme.border)),
        )
        .children(instances.iter().map(|instance| {
            let theme = cx.theme();
            render_instance_row(
                &state,
                instance,
                InstanceRowColors {
                    secondary_hover: theme.secondary_hover,
                    success: theme.success,
                    danger: theme.danger,
                    primary: theme.primary,
                    muted_foreground: theme.muted_foreground,
                    border: theme.border,
                },
            )
        }))
        .child(div().h_6())
}

fn scope_name(scope: &Option<ScopeData>, is_default: bool) -> String {
    if is_default {
        i18n::t("Default Scope").to_string()
    } else {
        scope
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "?".to_string())
    }
}

/// Theme colors used by an instance row.
struct InstanceRowColors {
    secondary_hover: gpui::Hsla,
    success: gpui::Hsla,
    danger: gpui::Hsla,
    primary: gpui::Hsla,
    muted_foreground: gpui::Hsla,
    border: gpui::Hsla,
}

fn render_instance_row(
    state: &crate::daemon::ServerState, instance: &InstanceData, colors: InstanceRowColors,
) -> impl IntoElement {
    let state = state.clone();
    let local = instance.local.clone();
    let local_copy = local.clone();
    let remote = instance.remote.clone();
    let label = instance.label.clone();
    let latency = instance.latency;

    let latency_text = if latency >= 0 {
        format!("{latency} ms")
    } else {
        "-- ms".to_string()
    };
    let latency_color = if latency >= 0 {
        colors.success
    } else {
        colors.danger
    };

    div()
        .id(format!("instance-{local}"))
        .rounded_md()
        .hover(|this| this.bg(colors.secondary_hover.opacity(0.4)))
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            RootView::copy_to_clipboard(cx, &local_copy);
        })
        .child(
            v_flex()
                .px_4()
                .py_2()
                .child(
                    h_flex()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .flex_1()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .child(label),
                        )
                        .child(div().text_color(colors.primary).child(local.to_string())),
                )
                .child(
                    h_flex()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .flex_1()
                                .text_color(colors.muted_foreground)
                                .child(remote),
                        )
                        .child(
                            h_flex()
                                .items_center()
                                .gap_2()
                                .child(div().text_color(latency_color).child(latency_text))
                                .child(
                                    Button::new(format!("close-{local}"))
                                        .flat()
                                        .small()
                                        .icon(
                                            Icon::new(IconName::Dismiss).text_color(colors.danger),
                                        )
                                        .on_click({
                                            let state = state.clone();
                                            let local = local.clone();
                                            move |_, _, cx| {
                                                cx.stop_propagation();
                                                let state = state.clone();
                                                let local = local.clone();
                                                daemon::tokio_handle().spawn(async move {
                                                    daemon::remove_instance(&state, &local).await;
                                                });
                                            }
                                        }),
                                ),
                        ),
                ),
        )
        .child(div().h_px().bg(colors.border))
}
