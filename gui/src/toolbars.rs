use gpui::*;

use crate::theme::THEME;

pub struct TitleBar;

impl Render for TitleBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .border_b_1()
            .border_color(THEME.divider)
            .window_control_area(WindowControlArea::Drag)
            .pl(px(71.))
            .bg(THEME.titlebar)
            .child("Project")
    }
}

pub struct ToolBar;

impl Render for ToolBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .border_b_1()
            .border_color(THEME.divider)
            .flex()
            .h(px(34.))
            .w_full()
            .bg(THEME.sidebar)
            .child("Tools")
    }
}

pub struct SideBar;

impl Render for SideBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .h_full()
            .w(px(200.))
            .border_r_1()
            .border_color(THEME.divider)
            .bg(THEME.sidebar)
            .child("Tools")
    }
}
