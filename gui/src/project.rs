use gpui::*;

use crate::{
    canvas::{test_canvas, LayoutCanvas},
    theme::THEME,
    toolbars::{SideBar, TitleBar, ToolBar},
};

pub struct LayerState {
    pub name: String,
    pub visible: bool,
}

pub struct Project {
    pub canvas: Entity<LayoutCanvas>,
    pub layers: Vec<LayerState>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let canvas = cx.new(|_cx| test_canvas());
        Self {
            canvas,
            layers: (0..10)
                .map(|i| LayerState {
                    name: format!("met{i}"),
                    visible: false,
                })
                .collect(),
        }
    }
}

impl Render for Project {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family("Zed Plex Sans")
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .items_start()
            .border_1()
            .border_color(THEME.divider)
            .rounded(px(10.))
            .text_sm()
            .text_color(rgb(0xffffff))
            .overflow_hidden()
            .child(cx.new(|_cx| TitleBar))
            .child(cx.new(|_cx| ToolBar))
            .child(
                div()
                    .flex()
                    .size_full()
                    .child(cx.new(|_cx| SideBar))
                    .child(self.canvas.clone()),
            )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {}
