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
    pub weight: usize,
    pub from: usize,
    pub to: usize,
}

impl Edge {
    fn new(weight: usize, from: usize, to: usize) -> Edge {
        Edge {weight, to, from}
    }
}

#[derive(Debug)]
struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
}

/*
    Unweighted Directed Graph
*/
impl Graph {
    fn new() -> Graph {
        Graph {
            vertexes: Vec::new(),
            edges: Vec::new(),
        }  
    }

    fn insert(&mut self, id: usize) {
        self.vertexes.push(Vertex::new(id));
    }

    fn connect(&mut self, to: usize, from: usize) {
        self.edges.push(Edge::new(1, to, from))
    }

    // fn distance(&self, v1: usize, v2: usize) -> usize {
        
    // }
}

fn main() {
    // let graph = Graph::new();

    let v = Vertex::new(4);
    println!("{:?}", v);

    let e = Edge::new(1, 2, 3);
    println!("{:?}", e);

    let mut g = Graph::new();
    println!("{:?}", g);
    g.insert(1);
    println!("{:?}", g);
    g.insert(2);
    println!("{:?}", g);
    g.insert(3);
    println!("{:?}", g);
    g.connect(1, 2);
    println!("{:?}", g);
    g.connect(2, 3);
    println!("{:?}", g);
}