use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::TcpStream;
use std::path::PathBuf;

use compiler::compile::{CompileInput, CompiledCell, compile};
use compiler::parse::parse;
use compiler::solver::Var;
use gpui::*;
use itertools::Itertools;

use crate::canvas::Rect;
use crate::socket::GuiToLsp;
use crate::{
    canvas::{LayoutCanvas, ShapeFill},
    theme::THEME,
    toolbars::{SideBar, TitleBar, ToolBar},
};

type Params = Vec<(String, f64)>;

/// Identifier for specific parametrizations of p-cell with name `name`.
struct CellId {
    name: String,
    params: Params,
}

/// Persistent state associated with a specific parametrization of a p-cell in a project.
pub struct CellState {
    pub rects: Vec<Rect<Var>>,
    pub solved_values: Vec<(Var, f64)>,
    // TODO: Use null space vectors to allow dragging coordinates.
    pub null_space: (),
    pub variable_overrides: Vec<(Var, f64)>,
}

/// Persistent state of project (i.e. anything that is saved in GUI project file).
///
/// GUI project file is saved in root directory of the associated Argon project.
pub struct ProjectState {
    pub root: PathBuf,
    pub code: String,
    /// Specific parametrizations of p-cells that have been compiled.
    pub cells: HashMap<CellId, CellData>,
    /// Cells that are open in the GUI.
    pub open_cells: Vec<CellId>,
    pub lsp_client: GuiToLsp<TcpStream>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>, lsp_client: GuiToLsp<TcpStream>) -> Self {
        Self {
            state,
            sidebar,
            canvas,
        }
    }
}

impl Project {
    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.canvas
            .update(cx, |canvas, cx| canvas.on_mouse_move(event, window, cx));
        cx.notify();
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
            .border_1()
            .border_color(THEME.divider)
            .rounded(px(10.))
            .text_sm()
            .text_color(rgb(0xffffff))
            .whitespace_nowrap()
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .child(cx.new(|_cx| TitleBar))
            .child(cx.new(|_cx| ToolBar))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .child(self.sidebar.clone())
                    .child(self.canvas.clone()),
            )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {}
