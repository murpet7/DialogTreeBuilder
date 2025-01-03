use eframe::egui::{Ui, Color32, Stroke};
use crate::dialog_tree::{DialogTree, Node};
use crate::input_manager::InputManager;
use crate::state_manager::State;

#[derive(Default)]
pub struct TreeRenderer {
    
}

impl TreeRenderer {
    pub fn update(&mut self, tree: &DialogTree, state: &State, input_manager: &InputManager, ui: &Ui) {
        let node_ids = tree.nodes.iter().map(|node| node.id);
        for id in node_ids {
            TreeRenderer::render_node(id, tree, input_manager, ui);
        }
        match &state {
            State::Dragging(node) => {
                TreeRenderer::render_node(*node, tree, input_manager, ui);
            }
            _ => { }
        }
    }

    fn render_node(node_id: usize, tree: &DialogTree, input_manager: &InputManager, ui: &Ui) {
        // Calculate the center of the panel
        // let pos = ctx.input(|i: &egui::InputState| i.screen_rect()).center();
    
        let mut color = Color32::BLUE;
        let hover_color = Color32::DARK_BLUE;
        
        match &input_manager.hovering_node_id {
            Some(hovering_node_id) => {
                if *hovering_node_id == node_id {
                    color = hover_color;
                }
            }
            None => { }
        }
        
        let node = tree.get_node_from_id(node_id);
        let stroke = Stroke::new(2.0, Color32::BLACK);
        match node {
            Some(node) => {
                ui.painter().circle(node.pos, node.rad, color, stroke);
            }
            None => eprintln!("Warning: Node ID of {} not found", node_id),
        }
    
        
        
    }
}

