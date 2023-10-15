use std::{collections::BinaryHeap, cmp::Ordering};
//use std::{collections::{HashMap, HashSet}};
use crate::chinakers::Field;


#[allow(dead_code)]
#[derive(Clone, Copy)]
struct AStarNode {
    id: usize,
    parent_id: usize,
    field: Field,
    heu_g: f32,
    heu_h: f32,
    heu: f32,
}


impl AStarNode {
    fn new() -> AStarNode {
        AStarNode {
            id: usize::MAX,
            parent_id: usize::MAX,
            field: Field::new(),
            heu_g: 0.0,
            heu_h: 0.0,
            heu: 0.0,
        }
    }

    fn from(field: Field) -> AStarNode {
        AStarNode {
            id: usize::MAX,
            parent_id: usize::MAX,
            field,
            heu_g: 0.0,
            heu_h: 0.0,
            heu: 0.0,
        }
    }
}

impl Default for AStarNode {
    fn default() -> Self {
        Self::new()
    }
}


impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl Eq for AStarNode {}


impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.heu.partial_cmp(&other.heu);
        if let Some(result_unwrapped) = result { Some(result_unwrapped.reverse()) } else { result }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(self.heu.partial_cmp(&other.heu).is_some());
        self.heu.partial_cmp(&other.heu).unwrap().reverse()
    }
}



pub struct AStarSolution {
    // pub moves: Vec<Move>,
    pub states: Vec<Field>,
}


pub struct AStar {
    nodes: Vec<AStarNode>,
    open: BinaryHeap<AStarNode>,
    // added: HashSet<u64>,
    // closed: HashMap<u64, usize>,
}

#[allow(unused)]
impl AStar {
    pub fn new(start: &Field) -> AStar {
        let mut astar = AStar {
            nodes: Default::default(),
            open: Default::default(),
            // added: Default::default(),
            // closed: Default::default(),
        };

        
        let mut node = AStarNode::new();
        node.field = start.clone();
        node.heu_h = start.eval_heuristic();
        node.heu_g = 0.0;
        node.heu = node.heu_h;
        node.id = astar.nodes.len();
        node.parent_id = astar.nodes.len();
        astar.nodes.push(node);
        astar.open.push(node);
        //astar.added.insert(start.unique_id());

        astar
    }

    pub fn step(&mut self) -> Option<Result<AStarSolution, ()>> {
        let Some(current_node) = self.open.pop() else { return Some(Err(())) };
        current_node.field.display(); println!("-------------");

        // Goal Reached
        if current_node.field.is_solved() {
            let mut node = &current_node;
            let mut prev_node_option: Option<&AStarNode> = None;
            let mut states = Vec::new();
            loop {
                states.push(node.field.clone());

                let next_node = self.nodes.get(node.parent_id);
                if next_node.is_none() { break }
                if node == next_node.unwrap() { break }
                prev_node_option = Some(&node);
                node = next_node.unwrap();
            }
            
            states.reverse();
            return Some(Ok(AStarSolution {states}));
        }

        // assert!(self.closed.get(&current_node.field.unique_id()).is_none(), "wtf is that = {:#01x}", current_node.field.unique_id());
        // self.closed.insert(current_node.field.unique_id(), current_node.id);

        current_node.field.available_moves().iter().for_each(|&m| {
            if !current_node.field.is_valid_move(m) { return }

            let mut field: Field = current_node.field.clone();
            let heu_g = current_node.heu_g + 1.0;
            let heu_h = field.eval_heuristic_for_move(m);
            let heu = heu_g + heu_h;
            field.make_move(m);
            
            // if self.added.contains(&field.unique_id()) { return }

            let mut node = AStarNode::from(field);
            node.id = self.nodes.len();
            node.parent_id = current_node.id;
            node.heu_g = heu_g;
            node.heu_h = heu_h;
            node.heu = heu;
            self.nodes.push(node);

            self.open.push(node);
            // self.added.insert(field.unique_id());
        });

        None
    }
}
