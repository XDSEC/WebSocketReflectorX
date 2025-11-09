// Sidebar view - Navigation sidebar
use gpui::{Context, Render, Window, div, prelude::*};
use crate::models::Page;
use crate::styles::colors;

pub struct SidebarView {
    active_page: Page,
}

impl SidebarView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, active_page: Page) -> Self {
        Self { active_page }
    }
    
    fn render_tab(&self, page: Page, label: impl Into<String>) -> impl IntoElement {
        let is_active = self.active_page == page;
        let label_text = label.into();
        div()
            .flex()
            .items_center()
            .px_4()
            .py_3()
            .when(is_active, |div| {
                div.bg(colors::accent())
            })
            .when(!is_active, |div| {
                div.hover(|div| div.bg(gpui::rgba(0x00000030)))
            })
            .child(label_text)
    }
}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .child(self.render_tab(Page::GetStarted, "Get Started"))
            .child(self.render_tab(Page::Connections, "Connections"))
            .child(self.render_tab(Page::NetworkLogs, "Network Logs"))
            .child(self.render_tab(Page::Settings, "Settings"))
    }
}

