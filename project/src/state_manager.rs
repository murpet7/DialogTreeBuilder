use crate::dialog_tree::DialogTree;

#[derive(Default)]
pub struct StateManager {
    pub state: State,
}

impl StateManager {

    pub fn check_state_transition(&mut self, tree: &mut DialogTree, state_transition: StateTransition, hovered_node_id: Option<usize>) {

        match (&self.state, state_transition) {
            (State::Dragging(_), StateTransition::LeftMBHold) => { }
            (_, StateTransition::LeftMBHold) => {
                match hovered_node_id {
                    Some(node_id) => {
                        self.state = State::Dragging(node_id);
                        tree.ordered_node_ids.retain(|id| *id != node_id);
                        tree.ordered_node_ids.push(node_id);
                    }
                    None => { }
                }
            }
            (State::Linking(_), StateTransition::RightMBHold) => { }
            (_, StateTransition::RightMBHold) => {
                match hovered_node_id {
                    Some(node_id) => {
                        self.state = State::Linking(node_id);
                        tree.create_link(node_id);
                    }
                    None => { }
                }
            }
            (State::Linking(node_id_from), StateTransition::NoButtonPress) => {
                match hovered_node_id {
                    Some(node_id_to) => {
                        tree.link_nodes(*node_id_from, node_id_to)
                    }
                    None => {
                        tree.remove_new_edge(*node_id_from)
                    }
                }
                self.state = State::Idle;
            }
            (_, StateTransition::NoButtonPress) => {
                self.state = State::Idle;
            }
            (_, _) => { }
        }
    }
}

pub enum State{
    Idle,
    Dragging(usize),
    Linking(usize),
    Editing,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

pub enum StateTransition {
    NoButtonPress,
    LeftMBHold,
    RightMBHold,
    LeftMBSinglePress,
}

