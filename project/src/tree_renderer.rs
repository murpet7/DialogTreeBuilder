use eframe::egui::{Ui, Color32, Stroke, Painter, Pos2};
use crate::dialog_tree::DialogTree;
use crate::input_manager::InputManager;
use crate::state_manager::State;

#[derive(Default)]
pub struct TreeRenderer {
    
}

impl TreeRenderer {
    pub fn update(&mut self, tree: &DialogTree, state: &State, input_manager: &InputManager, ui: &Ui) {
        for edge_id in tree.ids_to_edges.keys() {
            TreeRenderer::render_edge(*edge_id, tree, ui);
        }
        for node_id in &tree.ordered_node_ids {
            TreeRenderer::render_node(*node_id, tree, input_manager, ui);
        }
    }

    fn render_node(node_id: usize, tree: &DialogTree, input_manager: &InputManager, ui: &Ui) {
        // Calculate the center of the panel
        // let pos = ctx.input(|i: &egui::InputState| i.screen_rect()).center();
    
        let mut color = Color32::BLUE;
        let hover_color = Color32::DARK_BLUE;
        
        match &input_manager.hovered_node_id {
            Some(hovering_node_id) => {
                if *hovering_node_id == node_id {
                    color = hover_color;
                }
            }
            None => { }
        }
        
        let node = tree.get_node_from_id(node_id);
        let stroke = Stroke::new(2.0, Color32::BLACK);
        ui.painter().circle(node.pos, node.rad, color, stroke);
    }

    fn render_edge(edge_id: usize, tree: &DialogTree, ui: &Ui) {
        let edge = tree.get_edge_from_id(edge_id);
        let stroke = Stroke::new(2.0, Color32::BLACK);
        ui.painter().line_segment ([edge.pos_from, edge.pos_to], stroke);
    }
}

