mod graphs;

use graphs::graph_v1;

fn main() {
    let g: graph_v1::Graph::new();
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