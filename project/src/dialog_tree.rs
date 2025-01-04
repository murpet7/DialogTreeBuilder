use eframe::egui::Pos2;
use egui::Vec2;
use std::{collections::HashMap, ops::{Sub, SubAssign}, sync::atomic::{AtomicUsize, Ordering}, usize};

#[derive(Default)]
pub struct DialogTree {
    pub ids_to_nodes: HashMap<usize, Node>,
    pub ordered_node_ids: Vec<usize>,
    pub ids_to_edges: HashMap<usize, Edge>,
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub pos: Pos2,
    pub rad: f32,
    pub edges_from: Vec<usize>,
    pub edges_to: Vec<usize>,
}

static NEXT_ID_NODE: AtomicUsize = AtomicUsize::new(1);

impl Node {
    fn new(pos: Pos2, rad: f32) -> Self {
        let id = NEXT_ID_NODE.fetch_add(1, Ordering::SeqCst);
        Node { id, pos, rad, edges_from: Vec::new(), edges_to: Vec::new()}
    }
}

#[derive(Debug)]
pub struct Edge {
    pub id: usize,
    pub pos_from: Pos2,
    pub pos_to: Pos2,
    pub from_node: usize,
    pub to_node: Option<usize>,
}

static NEXT_ID_EDGE: AtomicUsize = AtomicUsize::new(1);

impl Edge {
    fn new(pos: Pos2, from_node: usize) -> Self {
        let id = NEXT_ID_EDGE.fetch_add(1, Ordering::SeqCst);
        Edge {id, pos_from: pos, pos_to: pos, from_node, to_node: None}
    }
}

impl DialogTree {
    pub fn add_node(&mut self, pointer_pos: Option<Pos2>) {
        let rad = 20.0;
        let new_node = Node::new(pointer_pos.unwrap(), rad);
        self.ordered_node_ids.push(new_node.id);
        self.ids_to_nodes.insert(new_node.id, new_node);
    }

    pub fn create_link(&mut self, node_id: usize) {
        let node = self.get_node_from_id_mut(node_id);
        let edge = Edge::new(node.pos, node_id);
        node.edges_from.push(edge.id);
        self.ids_to_edges.insert(edge.id, edge);
    }

    pub fn link_nodes(&mut self, node_id_from: usize, node_id_to: usize) {
        
        let edge_id = {
            let node_from = self.get_node_from_id(node_id_from);
            *node_from.edges_from.last().unwrap()
        };
    
        let node_to_pos = {
            let node_to = self.get_node_from_id(node_id_to);
            node_to.pos
        };
        
        let node_to = self.get_node_from_id_mut(node_id_to);
            node_to.edges_to.push(edge_id);
    
        {
            let edge = self.get_edge_from_id_mut(edge_id);
            edge.to_node = Some(node_id_to);
            edge.pos_to = node_to_pos;
        }
    }
    
    
    pub fn remove_new_edge(&mut self, node_id: usize) {
        let edge_id = {
            let node = self.get_node_from_id(node_id);
            *node.edges_from.last().unwrap()
        };
        self.ids_to_edges.remove(&edge_id);
        let node = self.get_node_from_id_mut(node_id);
        node.edges_from.retain(|from_id|*from_id != edge_id);
        
    }
    
    pub fn get_hovering_node(&mut self, pointer_pos: Option<Pos2>) -> Option<&Node> {
        let mut last_hovered_node = None;
        for node_id in self.ordered_node_ids.iter() {
            let node = self.get_node_from_id(*node_id);
            if DialogTree::is_hovering(&node, pointer_pos) {
                last_hovered_node = Some(node);
            }
        }
        return last_hovered_node;
    }

    pub fn get_node_from_id(&self, node_id: usize) -> &Node{
        self.ids_to_nodes.get(&node_id).unwrap()
    }

    pub fn get_node_from_id_mut(&mut self, node_id: usize) -> &mut Node{
        self.ids_to_nodes.get_mut(&node_id).unwrap()
    }

    pub fn get_edge_from_id(&self, edge_id: usize) -> &Edge{
        self.ids_to_edges.get(&edge_id).unwrap()
    }

    pub fn get_edge_from_id_mut(&mut self, edge_id: usize) -> &mut Edge{
        self.ids_to_edges.get_mut(&edge_id).unwrap()
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