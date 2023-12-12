#[derive(Debug)]
pub enum GraphError {
    NodeIndexOutOfBounds(NodeIndex),
    EdgeIndexOutOfBounds(EdgeIndex),
}

pub type NodeIndex = usize;

#[derive(Debug)]
struct Node<N> {
    id: NodeIndex,
    inner: N,
}

pub type EdgeIndex = usize;

#[derive(Debug)]
struct Edge<E> {
    id: EdgeIndex,
    inner: E,
    a: NodeIndex,
    b: NodeIndex,
}

#[derive(Debug)]
pub struct UndirectedGraph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> UndirectedGraph<N, E> {
    pub fn new() -> Self {
        return Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
    }

    pub fn get_node(&self, node: NodeIndex) -> Result<&N, GraphError> {
        return match self.nodes.get(node) {
            Some(n) => Ok(&n.inner),
            None => Err(GraphError::NodeIndexOutOfBounds(node)),
        };
    }

    pub fn get_mut_node(&mut self, node: NodeIndex) -> Result<&mut N, GraphError> {
        return match self.nodes.get_mut(node) {
            Some(n) => Ok(&mut n.inner),
            None => Err(GraphError::NodeIndexOutOfBounds(node)),
        };
    }

    pub fn get_edge(&self, edge: EdgeIndex) -> Result<&E, GraphError> {
        return match self.edges.get(edge) {
            Some(e) => Ok(&e.inner),
            None => Err(GraphError::EdgeIndexOutOfBounds(edge)),
        };
    }

    pub fn get_mut_edge(&mut self, edge: EdgeIndex) -> Result<&mut E, GraphError> {
        return match self.edges.get_mut(edge) {
            Some(e) => Ok(&mut e.inner),
            None => Err(GraphError::EdgeIndexOutOfBounds(edge)),
        };
    }

    pub fn node_count(&self) -> usize {
        return self.nodes.len();
    }

    pub fn edge_count(&self) -> usize {
        return self.edges.len();
    }

    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let id = self.nodes.len();
        self.nodes.push(Node { id, inner: data });
        return id;
    }

    pub fn add_edge(
        &mut self,
        a: NodeIndex,
        b: NodeIndex,
        data: E,
    ) -> Result<EdgeIndex, GraphError> {
        if self.get_node(a).is_err() {
            return Err(GraphError::NodeIndexOutOfBounds(a));
        }
        if self.get_node(b).is_err() {
            return Err(GraphError::NodeIndexOutOfBounds(b));
        }
        let id = self.edges.len();
        self.edges.push(Edge {
            id,
            inner: data,
            a,
            b,
        });
        return Ok(id);
    }

    pub fn degree(&self, node: NodeIndex) -> Result<usize, GraphError> {
        if self.get_node(node).is_err() {
            return Err(GraphError::NodeIndexOutOfBounds(node));
        }
        Ok(self
            .edges
            .iter()
            .filter(|edge| edge.a == node || edge.b == node)
            .count())
    }

    pub fn neighbors(&self, node: NodeIndex) -> Result<Neighbors<N, E>, GraphError> {
        if self.get_node(node).is_err() {
            return Err(GraphError::NodeIndexOutOfBounds(node));
        }

        return Ok(Neighbors {
            graph: self,
            node,
            current: 0,
        });
    }
}

pub struct Neighbors<'graph, N, E> {
    graph: &'graph UndirectedGraph<N, E>,
    node: NodeIndex,
    current: EdgeIndex,
}

impl<'graph, N, E> Iterator for Neighbors<'graph, N, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        while self.graph.edges.get(self.current).is_some() {
            self.current += 1;
            match self.graph.edges.get(self.current - 1) {
                Some(c) => {
                    if c.a == self.node {
                        return Some(c.b);
                    }
                    if c.b == self.node {
                        return Some(c.a);
                    }
                }
                None => unreachable!(),
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph:UndirectedGraph<usize,usize> = UndirectedGraph::new();
        println!("{:?}", graph);
        assert_eq!(0, graph.add_node(100));
        println!("{:?}", graph);
        assert_eq!(1, graph.add_node(200));
        println!("{:?}", graph);
        assert_eq!(100, *graph.get_node(0).unwrap());
        println!("{:?}", graph);
        assert_eq!(0, graph.add_edge(0, 1, 100).unwrap());
        println!("{:?}", graph);
    }
}