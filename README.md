# Graphs-in-Rust
Learning features of rust by implementing graphs with increasing complexity and robustness

## Graph v1
Graph is a struct with vec<Vertex> and vec<Edge>
Vertex is just an id: usize
Edge is unweighted directed edge with to: usize and from: usize

## Graph v2
Graph<T> is a struct with HashSet<Vertex<T>> and HashSet<Edge<T>>
vertexes are indexed with 
disallow duplicate vertexes and duplicate edges

## Ideas
- make graph a trait. look at javax.util.Graph for ideas of methods
- make graph, vertex, and edge generic
- make graph, vertex, and edge a trait so different implentations can be used
- MatrixGraph and ListGraph are Graph trait implementations
- WeightedUndirectedEdge, WeightedDirectedEdge, UndirectedEdge, DirectedEdge are edge implementations
- vertex requires an identifier and maybe needs default values
- vertex and edge must be hashable for graphs with set implementations
- be able to get an immutable graph from a graph
- be able to print graphs effectively
- implement different graph types with Big O analysis for standard operations
- be able to add graphs together
  - implement add trait to work like union of nodes and union of edges
  - any vertex with id1==id2 is treated as same vertex
  - SCC constructor that takes a list of nodes and returns a graph with nodes full connected
  - can construct graphs efficiently by defining a graph as the union of other graphs
    - scc() returns a graph with edges filled in between every node
    - ring() returns a graph with edges between each consecutive node including an edge from last to first
    - let g: Graph = Graph::scc('a', 'b', 'c') + Graph::ring('a', 'd', 'e')
    
