use gpui::{
    div, App, BorderStyle, Bounds, Context, Corners, DefiniteLength, Edges, Element, Entity,
    InteractiveElement, IntoElement, Length, MouseButton, MouseDownEvent, PaintQuad, ParentElement,
    Pixels, Point, Render, Rgba, Size, Style, Styled, Window,
};

#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}

// ~TextElement
pub struct CanvasElement {
    inner: Entity<LayoutCanvas>,
}

// ~TextInput
pub struct LayoutCanvas {
    pub offset: Point<Pixels>,
    pub rects: Vec<Rect>,
    pub bg_style: Style,
}

impl IntoElement for CanvasElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for CanvasElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let inner = self.inner.read(cx);
        let layout_id = window.request_layout(inner.bg_style.clone(), [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) {
        let inner = self.inner.read(cx);
        let rects = inner.rects.clone();
        let offset = inner.offset;
        inner
            .bg_style
            .clone()
            .paint(bounds, window, cx, |window, cx| {
                for r in rects {
                    let bounds = Bounds::new(
                        Point::new(Pixels(r.x0), Pixels(r.y0)) + offset + bounds.origin.clone(),
                        Size::new(Pixels(r.x1 - r.x0), Pixels(r.y1 - r.y0)),
                    );
                    let color = Rgba {
                        r: 1.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    };
                    window.paint_quad(PaintQuad {
                        bounds,
                        corner_radii: Corners::all(Pixels(0.)),
                        background: color.into(),
                        border_widths: Edges::all(Pixels(0.)),
                        border_color: color.into(),
                        border_style: BorderStyle::Solid,
                    });
                }
            });
    }
}

impl Render for LayoutCanvas {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down_listener))
            .child(CanvasElement {
                inner: cx.entity().clone(),
            })
    }
}

pub(crate) fn test_canvas() -> LayoutCanvas {
    LayoutCanvas {
        rects: vec![Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 100.,
            y1: 40.,
        }],
        offset: Point::new(Pixels(0.), Pixels(0.)),
        bg_style: Style {
            size: Size {
                width: Length::Definite(DefiniteLength::Fraction(1.)),
                height: Length::Definite(DefiniteLength::Fraction(1.)),
            },
            ..Style::default()
        },
    }
}

impl LayoutCanvas {
    fn on_mouse_down(event: &MouseDownEvent, window: &mut Window, cx: &mut App) {
        println!("mouse down");
    }
    fn on_mouse_down_listener(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("mouse down");
        cx.notify();
    }
}
