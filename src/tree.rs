use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct Tree<T> {
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

    pub fn nodes(&self) -> &HashMap<usize, Node<T>> {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut HashMap<usize, Node<T>> {
        &mut self.nodes
    }

    pub fn set_root_id(&mut self, id: usize) {
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

    pub fn add_node_with_id(&mut self, id: usize, data: T) {
        self.nodes.insert(id, Node::new(data));
    }

    pub fn remove_node(&mut self, id: usize) -> Option<Node<T>> {
        let node = self.nodes.remove(&id);
        if let Some(node) = &node {
            self.remove_children(node);
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

    pub fn get(&self, id: usize) -> Option<&Node<T>> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(&id)
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        if let Some(node) = self.nodes.get_mut(&parent) {
            node.children.push(child)
        }
    }
}

impl<T> Index<usize> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.get(&index).unwrap()
    }
}

impl<T> IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.nodes.get_mut(&index).unwrap()
    }
}

pub struct Node<T> {
    pub data: T,
    pub children: Vec<usize>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            children: Vec::new(),
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

pub static NEXT_ELEMENT_ID: AtomicUsize = AtomicUsize::new(0);
pub fn next_element_id() -> usize {
    NEXT_ELEMENT_ID.fetch_add(1, Ordering::SeqCst) + 1
}
