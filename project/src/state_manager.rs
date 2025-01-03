#[derive(Default)]
pub struct StateManager {
    pub state: State,
}

impl StateManager {

    pub fn check_state_transition(&mut self, state_transition: StateTransition, hovering_node: Option<usize>) {

        match (&self.state, state_transition) {
            (State::Dragging(_), StateTransition::LeftMBHold) => { }
            (_, StateTransition::LeftMBHold) => {
                match hovering_node {
                    Some(node) => {
                        self.state = State::Dragging(node)
                    }
                    None => { }
                }
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

