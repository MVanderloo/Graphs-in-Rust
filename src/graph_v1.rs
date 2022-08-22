mod graph_v1 {
    /**
     * A very simple unweighted directed graph.
     * Edge { from: usize, to: usize }
     * Graph has methods insert and connect to create vertexes and edges respectively.
     * Also has methods to 
     * It is easy to reach an invalid state as everything is just inserted as is.
     * Problems:
     * - duplicate vertexes
     * - duplicate edges
     * - id limited to usize
    **/

    #[derive(Debug)]
    struct Vertex {
        pub id: usize
    }

    impl Vertex {
        fn new(id: usize) -> Vertex {
            Vertex {id}
        }
    }

    #[derive(Debug)]
    struct Edge {
        pub from: usize,
        pub to: usize,
    }

    impl Edge {
        fn new(from: usize, to: usize) -> Edge {
            Edge {to, from}
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        vertexes: Vec<Vertex>,
        edges: Vec<Edge>,
    }

    /*
        Unweighted Directed Graph
    */
    impl Graph {
        pub fn new() -> Graph {
            Graph {
                vertexes: Vec::new(),
                edges: Vec::new(),
            }  
        }

        fn insert(&mut self, id: usize) {
            self.vertexes.push(Vertex::new(id));
        }

        fn remove(&mut self, id: usize) {
            for (i, v) in self.vertexes.iter().enumerate() {
                if v.id == id {
                    self.vertexes.swap_remove(i);
                }
            }
        }

        fn connect(&mut self, to: usize, from: usize) {
            self.edges.push(Edge::new(to, from))
        }

        fn disconnect(&mut self, to: usize, from: usize) {
            for (i, e) in self.edges.iter().enumerate() {
                if e.to == to && e.from == from {
                    self.edges.swap_remove(i);
                }
            }
        }
    }
}

use graph_v1::Graph;

fn main() {
    let g: Graph::new();
    println!("{:?}", g);
    g.insert(0);
    println!("{:?}", g);
    g.remove(0);
    println!("{:?}", g);
    g.insert(1);
    println!("{:?}", g);
    g.remove(2);
    println!("{:?}", g);
    g.insert(2);
    println!("{:?}", g);
    g.connect(1, 2);
    println!("{:}?", g);
    g.disconnect(1, 2);
    println!("{:?}", g);
    g.connect(2, 1);
    println!("{:?}", g);
    g.insert(3);
    println!("{:?}", g);
    g.insert(3);
    println!("{:?}", g);
    g.connect(3, 3);
    println!("{:?}", g);
    g.disconnect(2, 3);
    println!("{:?}", g);
}