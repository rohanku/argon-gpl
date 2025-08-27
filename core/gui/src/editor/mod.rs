pub struct LayerState {
    pub name: String,
    pub color: Rgba,
    pub fill: ShapeFill,
    pub border_color: Rgba,
    pub visible: bool,
    pub z: usize,
}

pub struct EditorState {
    pub selected_rect: Option<usize>,
    pub layers: Vec<Entity<LayerState>>,
    pub subscriptions: Vec<Subscription>,
}

pub struct Editor {
    pub project: Entity<ProjectState>,
    pub sidebar: Entity<SideBar>,
    pub canvas: Entity<LayoutCanvas>,
}

impl Editor {
    pub fn new(cx: &mut Context<Self>, lsp_client: GuiToLsp<TcpStream>) -> Self {
        let ast = parse(&code).expect("failed to parse Argon");
        let params_ref = params.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        let solved_cell = compile(CompileInput {
            cell: &cell,
            ast: &ast,
            params: params_ref,
        });
        let layers: HashSet<_> = solved_cell
            .values
            .iter()
            .filter_map(|value| value.get_rect()?.layer.clone())
            .collect();
        let layers: Vec<_> = layers
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(z, name)| {
                let mut s = DefaultHasher::new();
                name.hash(&mut s);
                let hash = s.finish() as usize;
                let color = rgb([0xff0000, 0x0ff000, 0x00ff00, 0x000ff0, 0x0000ff][hash % 5]);
                cx.new(|_cx| LayerState {
                    name,
                    color,
                    fill: ShapeFill::Stippling,
                    border_color: color,
                    visible: true,
                    z,
                })
            })
            .collect();
        let rects = get_rects(cx, &solved_cell, &layers);
        let state = cx.new(|cx| {
            let subscriptions = layers
                .iter()
                .map(|layer| {
                    cx.observe(layer, |_, _, cx| {
                        println!("project notified");
                        cx.notify();
                    })
                })
                .collect();
            ProjectState {
                path,
                code,
                cell,
                params,
                solved_cell: solved_cell.clone(),
                rects,
                selected_rect: None,
                layers,
                subscriptions,
                lsp_client,
            }
        });

        let sidebar = cx.new(|cx| SideBar::new(cx, &state));
        let canvas = cx.new(|cx| LayoutCanvas::new(cx, &state));
    }
}
