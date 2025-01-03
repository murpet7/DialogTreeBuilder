use eframe::egui::Context;
use crate::state_manager::{StateManager, State, StateTransition};
use crate::dialog_tree::{DialogTree, Node};

#[derive(Default)]
pub struct InputManager {
    pub hovering_node_id: Option<usize>,
}

impl InputManager {

    pub fn update(&mut self, tree: &mut DialogTree, state_manager: &mut StateManager, ctx: &Context) {

        match state_manager.state {
            State::Idle => {
                InputManager::check_add_node(tree, ctx);
            }
            State::Dragging(node_id) => {
                let mut node = tree.get_node_from_id_mut(node_id);
                match &mut node {
                    Some(node) => {
                        node.pos = ctx.pointer_hover_pos().unwrap();
                    }
                    None => eprintln!("Warning: Node ID of {} not found", node_id),
                } 
            }
            _ => { }
        }
        self.check_state_transition(tree, state_manager, &ctx);
        self.update_hovered_node(tree, ctx);
    }

    fn update_hovered_node(&mut self, tree: &mut DialogTree, ctx: &Context) {
        let hovering_node = tree.get_hovering_node(ctx.pointer_hover_pos());
        self.hovering_node_id = hovering_node.map(|node| node.id);
    }

    fn check_add_node(tree: &mut DialogTree, ctx: &Context) {
        if ctx.input(|i| i.pointer.button_double_clicked(egui::PointerButton::Primary)) {
            let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
            tree.add_node(pointer_pos);
        }
    }

    fn check_state_transition(&mut self, tree: &mut DialogTree, state_manager: &mut StateManager, ctx: &Context) {
        let state_transition = InputManager::get_state_transition(ctx);
        state_manager.check_state_transition(tree, state_transition, self.hovering_node_id);
    }

    fn get_state_transition(ctx: &Context) -> StateTransition {
        if ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)){
            return StateTransition::LeftMBSinglePress;
        }
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary)){
            return StateTransition::LeftMBHold;
        }
        if ctx.input(|i| i.pointer.button_down(egui::PointerButton::Secondary)){
            return StateTransition::RightMBHold;
        }
        return StateTransition::NoButtonPress;
    }
}