use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, px, prelude::FluentBuilder as _};
use woocraft::{
    ActiveTheme, Anchor, Button, ButtonVariants as _, Icon, IconName, Input, Popover,
    ScrollableElement as _, Selectable as _, Sizable as _, h_flex, v_flex,
};

use crate::{daemon, i18n, models::InstanceData, ui::RootView};

/// Renders the "Get Started" home page.
pub(crate) fn render_get_started(
    _window: &mut Window,
    cx: &mut Context<RootView>,
    root: &mut RootView,
) -> impl IntoElement {
    let theme = cx.theme();
    let weak = cx.entity().downgrade();
    let state = root.state().clone();

    let has_updates = root.has_updates();
    let cursor = if root.cursor_visible() { "_" } else { " " };
    let interfaces = root.interfaces().to_vec();
    let interface_input = root.interface_input().clone();
    let remote_input = root.remote_input().clone();
    let port_input = root.port_input().clone();

    v_flex()
        .gap_1()
        .px_10()
        .py_6()
        .min_h_full()
        .items_center()
        .justify_center()
        .overflow_y_scrollbar()
        .size_full()
        .child(
            // Header: logo + title + subtitle
            v_flex()
                .items_center()
                .gap_1()
                .child(Icon::new(IconName::GlobeStar).size(px(64.)).text_color(theme.primary))
                .child(
                    h_flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()

                                .font_weight(gpui::FontWeight::BOLD)
                                .child("WebSocket Reflector X"),
                        )
                        .when(has_updates, |this| {
                            this.child(
                                Button::new("update")
                                    .flat()
                                    .small()
                                    .icon(Icon::new(IconName::Sparkle))
                                    .label(i18n::t("Update"))
                                    .on_click(|_, _, _| {
                                        RootView::open_link(
                                            "https://github.com/XDSEC/WebSocketReflectorX/releases",
                                        );
                                    }),
                            )
                        }),
                )
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child(format!(
                            "{}{cursor}",
                            i18n::t(
                                "Controlled TCP-over-WebSocket forwarding tunnel"
                            )
                        )),
                ),
        )
        .child(
            // Form: local interface + port, remote address + send
            v_flex()
                .w(px(520.))
                .gap_1()
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("refresh-interfaces")
                                .outline(true)
                                .icon(Icon::new(IconName::ArrowSync))
                                .on_click({
                                    let weak = weak.clone();
                                    move |_, _, cx| {
                                        let _ = weak.update(cx, |root, cx| {
                                            root.refresh_interfaces();
                                            cx.notify();
                                        });
                                    }
                                }),
                        )
                        .child(
                            // Local address input with a popup interface picker.
                            Popover::new("interface-popover")
                                .anchor(Anchor::BottomLeft)
                                .overlay_closable(true)
                                .trigger(
                                    Input::new(&interface_input)
                                        .flex_1()
                                        .bordered(true),
                                )
                                .content({
                                    let interfaces = interfaces.clone();
                                    let input = interface_input.clone();
                                    move |_, _window, cx| {
                                        let state_entity = cx.entity();
                                        let current = input.read(cx).value().to_string();
                                        v_flex()
                                            .gap_1()
                                            .p_1()
                                            .w(px(260.))
                                            .children(interfaces.iter().map(|interface| {
                                                let interface = interface.clone();
                                                let input = input.clone();
                                                let state_entity = state_entity.clone();
                                                Button::new(format!("iface-{interface}"))
                                                    .flat()
                                                    .expand(true)
                                                    .selected(interface == current)
                                                    .label(interface.clone())
                                                    .on_click(move |_, window, cx| {
                                                        input.update(cx, |input, input_cx| {
                                                            input.set_value(
                                                                interface.clone(),
                                                                window,
                                                                input_cx,
                                                            );
                                                        });
                                                        let _ = state_entity
                                                            .update(cx, |state, cx| {
                                                                state.dismiss(window, cx);
                                                            });
                                                    })
                                            }))
                                    }
                                }),
                        )
                        .child(Input::new(&port_input).w(px(90.))),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Input::new(&remote_input)
                                .flex_1()
                                .bordered(true),
                        )
                        .child(
                            Button::new("send")
                                .primary()
                                .icon(Icon::new(IconName::Send))
                                .on_click({
                                    let weak = weak.clone();
                                    let state = state.clone();
                                    let interface_input = interface_input.clone();
                                    move |_, _, cx| {
                                        let state = state.clone();
                                        let interface =
                                            interface_input.read(cx).value().to_string();
                                        let remote = remote_input.read(cx).value().to_string();
                                        let port = port_input.read(cx).value().to_string();
                                        let local = format!("{interface}:{port}");

                                        daemon::tokio_handle().spawn(async move {
                                            let data = InstanceData {
                                                label: daemon::default_label(),
                                                remote,
                                                local,
                                                latency: -1,
                                                scope_host: "default-scope".to_string(),
                                            };
                                            let _ =
                                                daemon::launch_instance(&state, &data).await;
                                        });

                                        let _ = weak.update(cx, |root, cx| {
                                            root.change_page(
                                                crate::ui::Page::Scope(
                                                    "default-scope".to_string(),
                                                ),
                                                cx,
                                            );
                                        });
                                    }
                                }),
                        ),
                ),
        )
}
