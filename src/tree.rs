use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::geo::{Point, Rect};

pub type ElementId = usize;

pub struct Tree<T: Sized> {
    nodes: HashMap<usize, Node<T>>,
    root: usize,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            root: Default::default(),
        }
    }
}

impl<T> Tree<T> {
    pub fn new(data: T) -> Self {
        let mut tree = Self {
            nodes: HashMap::new(),
            root: 0,
        };

        let root = tree.add_node(data);
        tree.root = root;
        tree
    }

    pub fn new_with_root_id(data: T, root: ElementId) -> Self {
        let mut tree = Self {
            nodes: HashMap::new(),
            root,
        };

        tree.add_node_with_id(root, Node::new(data));
        tree
    }

    pub fn new_with_root_node(node: Node<T>, id: ElementId) -> Self {
        let mut tree = Self {
            nodes: HashMap::new(),
            root: id,
        };

        tree.add_node_with_id(id, node);
        tree
    }

    pub fn root_mut(&mut self) -> &mut Node<T> {
        let id = self.root_id();
        &mut self[id]
    }

    pub fn consume_nodes(self) -> HashMap<usize, Node<T>> {
        self.nodes
    }

    pub fn nodes(&self) -> &HashMap<usize, Node<T>> {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut HashMap<usize, Node<T>> {
        &mut self.nodes
    }

    pub fn set_root_id(&mut self, id: ElementId) {
        self.root = id
    }

    pub fn root_id(&self) -> usize {
        self.root
    }

    pub fn add_node(&mut self, data: T) -> usize {
        let id = next_element_id();
        self.nodes.insert(id, Node::new(data));
        id
    }

    pub fn add_node_with_id(&mut self, id: ElementId, node: Node<T>) {
        self.nodes.insert(id, node);
    }

    pub fn remove_node(&mut self, id: ElementId) -> Node<T> {
        let node = self.nodes.remove(&id).unwrap();

        self.remove_children(&node);

        if let Some(parent) = self.find_parent(id) {
            self.remove_child_from_parent(parent, id)
        }

        node
    }

    fn remove_children(&mut self, parent: &Node<T>) {
        for child_id in &parent.children {
            if let Some(child) = self.nodes.remove(child_id) {
                self.remove_children(&child)
            }
        }
    }

    pub fn get(&self, id: ElementId) -> Option<&Node<T>> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(&id)
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        if let Some(node) = self.nodes.get_mut(&parent) {
            node.children.push(child);
            node.children.sort();
        }
    }

    pub fn remove_child_from_parent(&mut self, parent: usize, child: usize) {
        if let Some(node) = self.nodes.get_mut(&parent) {
            if let Some(index) = node.children.iter().position(|&element| element == child) {
                node.children.remove(index);
            }
        }
    }

    pub fn find_parent(&self, child: usize) -> Option<usize> {
        for (id, node) in &self.nodes {
            if node.children.contains(&child) {
                return Some(*id);
            }
        }

        None
    }

    /// Returns all newly added element id's
    pub fn merge_subtree(&mut self, parent: ElementId, subtree: Self) -> Vec<ElementId> {
        let mut results = Vec::new();
        self.add_child(parent, subtree.root_id());
        for (id, node) in subtree.consume_nodes() {
            self.add_node_with_id(id, node);
            results.push(id)
        }

        results
    }
}

#[cfg(feature = "dot")]
impl<T> Tree<T> {
    pub fn dot(&self) -> ellipsis::Dot {
        let mut graph = ellipsis::Graph::new(None);

        self.add_node_to_dot_graph(self.root, &mut graph);

        ellipsis::Dot::new(false, graph)
    }

    fn add_node_to_dot_graph(&self, id: ElementId, graph: &mut ellipsis::Graph) {
        if let Some(node) = self.nodes.get(&id) {
            graph.nodes.push(ellipsis::Node::new(format!("{id}")));
            for child in &node.children {
                self.add_node_to_dot_graph(*child, graph);
                graph
                    .edges
                    .push(ellipsis::Edge::new(format!("{id}"), format!("{child}")));
            }
        }
    }
}

impl<T> Index<ElementId> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: ElementId) -> &Self::Output {
        self.nodes.get(&index).unwrap()
    }
}

impl<T> IndexMut<ElementId> for Tree<T> {
    fn index_mut(&mut self, index: ElementId) -> &mut Self::Output {
        self.nodes.get_mut(&index).unwrap()
    }
}

pub struct Node<T: Sized> {
    pub data: T,
    pub children: Vec<usize>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            children: Vec::new(),
            data,
            global_bounds: Rect::default(),
            local_bounds: Rect::default(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn set_bounds(&mut self, rect: &Rect) {
        self.local_bounds = *rect;
        self.global_bounds = *rect;
    }

    pub fn hit_test(&self, point: &Point) -> bool {
        self.global_bounds.hit_test(point)
    }
}

pub static NEXT_ELEMENT_ID: AtomicUsize = AtomicUsize::new(0);
pub fn next_element_id() -> usize {
    NEXT_ELEMENT_ID.fetch_add(1, Ordering::SeqCst) + 1
}
