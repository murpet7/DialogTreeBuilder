use eframe::egui::{self, Color32, Pos2, Stroke, Ui, Context};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Interactable Circle Example",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(DialogTreeBuilder::new(cc)))),
    )
}

#[derive(Default)]
struct DialogTreeBuilder {
    state: States,
    nodes: Vec<Node>,
}



impl Default for States {
    fn default() -> Self {
        States::Idle
    }
}

#[derive(Debug)]
struct Node {
    pos: Pos2,
    rad: f32,
    links: Vec<Link>,
}

impl Node {
    fn new(pos: Pos2, rad: f32) -> Self {
        Node { pos, rad, links: Vec::new()}
    }
}

#[derive(Debug)]
struct Link {
    pos: Pos2,
    rot: f32,
    len: f32,
    from_node: Node,
    to_node: Node,
}

enum StateTransitions {
    NoButtonPress,
    LeftMBHold,
    RightMBHold,
    LeftMBSinglePress,
}

impl DialogTreeBuilder {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    
}

impl eframe::App for DialogTreeBuilder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            manage_tree(self, ctx, ui);
        });
    }
}

fn manage_tree(app: &mut DialogTreeBuilder, ctx: &Context, ui: &Ui) {
    for node in &app.nodes {
        render_node(&node, ctx, ui);
    }
    match &mut app.state {
        States::Idle => {
            check_add_node(app, ctx);
        }
        States::Dragging(node) => {
            node.pos = ctx.pointer_hover_pos().unwrap();
            render_node(node, ctx, ui);
        }
        States::Linking(node) => {
            // Logic for linking state
        }
        States::Editing => {
            // Logic for editing state
        }
    }
    check_state_transition(app, &ctx);
}

fn check_state_transition(app: &mut DialogTreeBuilder, ctx: &Context) {
    let state_transition = get_state_transition(&ctx);

    match (&app.state, state_transition) {
        (_, StateTransitions::LeftMBHold) => {
            let hovering_node = get_hovering_node(app, ctx);
            match hovering_node {
                Some(node) => {
                    app.state = States::Dragging(node)
                }
                None => { }
            }
        }
        (States::Dragging(node), StateTransitions::NoButtonPress) => {
            app.nodes.push(node);
            app.state = States::Idle
        }
        (_, StateTransitions::NoButtonPress) => {
            app.state = States::Idle;
        }
        (_, _) => { }
    }
}

fn get_state_transition(ctx: &Context) -> StateTransitions {
    if ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)){
        return StateTransitions::LeftMBSinglePress;
    }
    if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary)){
        return StateTransitions::LeftMBHold;
    }
    if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Secondary)){
        return StateTransitions::RightMBHold;
    }
    return StateTransitions::NoButtonPress;
}



fn render_node(node: &Node, ctx: &Context, ui: &Ui) {
    // Calculate the center of the panel
    // let pos = ctx.input(|i: &egui::InputState| i.screen_rect()).center();

    let mut color = Color32::BLUE;
    let hover_color = Color32::DARK_BLUE;

    if is_hovering(&node, &ctx) {
        color = hover_color;
    }

    let stroke = Stroke::new(2.0, Color32::BLACK);
    ui.painter().circle(node.pos, node.rad, color, stroke);
}

fn check_add_node(app: &mut DialogTreeBuilder, ctx: &Context) {
    if ctx.input(|i| i.pointer.button_double_clicked(egui::PointerButton::Primary)) {
        let pos = ctx.input(|i| i.pointer.hover_pos());
        let rad = 20.0;
        app.nodes.push(Node::new(pos.unwrap(), rad));
    }
}

fn get_hovering_node(app: &mut DialogTreeBuilder, ctx: &Context) -> Option<Node> {
    // Move the elements out of the vector using `into_iter()`
    for node in app.nodes.drain(..).rev() {
        if is_hovering(&node, ctx) {
            return Some(node); // Move the node out of the vector
        }
    }
    None
}

fn is_hovering(node: &Node, ctx: &Context) -> bool {
    let pointer_pos = ctx.pointer_hover_pos();
    let is_hovering = pointer_pos
        .map(|ppos| (ppos - node.pos).length() <= node.rad)
        .unwrap_or(false);
    return is_hovering;
}