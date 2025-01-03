use eframe::egui::Pos2;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct DialogTree {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub pos: Pos2,
    pub rad: f32,
    pub edges: Vec<Edge>,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

impl Node {
    fn new(pos: Pos2, rad: f32) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Node { id, pos, rad, edges: Vec::new()}
    }
}

#[derive(Debug)]
struct Edge {
    pos: Pos2,
    rot: f32,
    len: f32,
    from_node: Node,
    to_node: Node,
}

impl DialogTree {
    pub fn add_node(&mut self, pointer_pos: Option<Pos2>) {
        let rad = 20.0;
        self.nodes.push(Node::new(pointer_pos.unwrap(), rad));
    }
    
    pub fn get_hovering_node(&mut self, pointer_pos: Option<Pos2>) -> Option<&Node> {
        let mut last_hovered_node = None;
        for node in &self.nodes {
            if DialogTree::is_hovering(&node, pointer_pos) {
                last_hovered_node = Some(node);
            }
        }
        return last_hovered_node;
    }

    pub fn get_node_from_id(&self, node_id: usize) -> Option<&Node>{
        self.nodes.iter().find(|&node| node.id == node_id)
    }

    pub fn get_node_from_id_mut(&mut self, node_id: usize) -> Option<&mut Node>{
        self.nodes.iter_mut().find(|node| node.id == node_id)
    }
    
    pub fn is_hovering(node: &Node, pointer_pos: Option<Pos2>) -> bool {
        match pointer_pos {
            Some(pointer_pos) => {
                return pointer_pos.distance(node.pos) <= node.rad;
            }
            None => false,
        }
    }
}