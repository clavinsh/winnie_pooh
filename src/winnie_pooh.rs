
use crate::parser::Parser;

use rand::seq::SliceRandom;
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    vec,
};

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Edge {
    pub(crate) u: u32,
    pub(crate) v: u32,
    pub(crate) w: i8,
}

impl Edge {
    pub fn is_equal(&self, other: &Self) -> bool {
        (self.u == other.u && self.v == other.v && self.w == other.w)
            || self.u == other.v && self.v == other.u && self.w == other.w
    }

    // normalizē šķautni, lai abas (u,v) un (v,u) versijas būtu vienādas
    fn normalized(&self) -> Self {
        if self.u <= self.v {
            *self
        } else {
            Edge {
                u: self.v,
                v: self.u,
                w: self.w,
            }
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub(crate) edge_list: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edge_list: vec![] }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edge_list.push(edge);
    }

    pub fn sort_by_weight_desc(&mut self) {
        self.edge_list.sort_by_key(|e| std::cmp::Reverse(e.w));
    }

    // ievades dati formātā:
    // n a_1 b_1 w_1 a_2 b_2 w_2 ... a_m b_m w_m,
    //
    // n - virsotņu skaits grafā (n < 5000)
    // a_i, b_i iekš {1, ..., n}
    // w_i iekš {-99, ... 99}
    // a_i b_i w_1 reprezentē šķautni starp virsotnēm a,b, ar svaru w
    // šķautni var reprezetnēt gan kā a_i, b_i, w_i, gan b_i, a_i, w_i
    pub fn from_input(input: String) -> Graph {
        let mut parser = Parser::new(input);

        let mut graph = Graph::new();

        parser.consume_whitespace();

        let n = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u32>()
            .unwrap();

        assert!(n > 0 && n < 5000);

        parser.consume_whitespace();

        while !parser.eof() {
            let a = parser
                .next_while(|c| !c.is_whitespace())
                .parse::<u32>()
                .unwrap();

            parser.consume_whitespace();
            let b = parser
                .next_while(|c| !c.is_whitespace())
                .parse::<u32>()
                .unwrap();

            parser.consume_whitespace();
            let w = parser
                .next_while(|c| !c.is_whitespace())
                .parse::<i8>()
                .unwrap();

            graph.add_edge(Edge { u: a, v: b, w: w });

            parser.consume_whitespace();
        }

        return graph;
    }

    pub fn invert_edge_weights(&mut self) {
        for edge in &mut self.edge_list {
            edge.w *= -1;
        }
    }

    pub fn sort_by_weight_asc(&mut self) {
        self.edge_list.sort_by_key(|e| e.w);
    }

    // testēšanai
    pub fn randomize(&mut self) {
        let mut rng = rand::rng();
        self.edge_list.shuffle(&mut rng);
    }

    // max_weight_span_tree_kruskal
    // grafam obligāti jau jābūt sakārtotam
    pub fn mst_kruskal(&self) -> Graph {
        let mut disjoint_forest = DisjointForest::new();
        let mut mst = Graph::new();

        for edge in &self.edge_list {
            // varētu disjoint foresta izveidi iznest fn 'from_input' kopā ar pašu grafu, kas būtu efektīvāk, bet
            // idejiski kopējā sarežģītība nemainās, jo šī sadaļa ir O(|E|), contains_key funkcija HashMap struktūrām ir O(1)
            if !disjoint_forest.forest.contains_key(&edge.u) {
                disjoint_forest.make_set(edge.u);
            }

            if !disjoint_forest.forest.contains_key(&edge.v) {
                disjoint_forest.make_set(edge.v);
            }
        }

        for edge in &self.edge_list {
            if disjoint_forest.find_set(edge.u) != disjoint_forest.find_set(edge.v) {
                mst.add_edge(edge.clone());
                disjoint_forest.union(edge.u, edge.v);
            }
        }

        return mst;
    }

    pub fn graph_weight_sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for edge in &self.edge_list {
            sum += i32::from(edge.w);
        }
        return sum;
    }

    pub fn serialize_edges(&self) -> String {
        let mut serialized = String::new();

        let k = self.edge_list.len();
        let w = self.graph_weight_sum();

        writeln!(serialized, "{} {}", k, w).expect("Failed to serialize graph to a string");

        for edge in &self.edge_list {
            writeln!(serialized, "{} {}", edge.u, edge.v)
                .expect("Failed to serialize graph to a string");
        }

        return serialized;
    }
}

macro_rules! no_parent_err {
    ($v:expr) => {
        format!("Value {} does not have a parent!", $v)
    };
}

// Disjoint-set forest implementācija ar ranka un path compression optimizācijām
struct DisjointForest {
    // key - virosotnes id,
    // value - parent virsotnes id
    //               key  rank  value
    //                ↓     ↓    ↓
    forest: HashMap<u32, (u32, u32)>,
}

impl DisjointForest {
    pub fn new() -> DisjointForest {
        DisjointForest {
            forest: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, v: u32) {
        self.forest.insert(v, (0, v));
    }

    // pārveidojam katras virsotnes parent ķēdē par root
    // ja veidojas degenerate grafs, tad tas tiks saspiests
    // no:
    //        root
    //         │
    //         v3
    //         │
    //         v2
    //         │
    //         v1
    // uz:
    //        root
    //       / | \
    //     v1  v2 v3
    pub fn find_set(&mut self, v: u32) -> (u32, u32) {
        let (rank, root) = self.find_root(v);

        if v != root {
            let mut current = v;
            while current != root {
                let (_rank, parent) = self
                    .forest
                    .get_mut(&current)
                    .expect(&no_parent_err!(current));

                *parent = root;

                current = *parent;
            }
        }

        return (rank, root);
    }

    pub fn find_root(&self, v: u32) -> (u32, u32) {
        let mut current = v;

        let root = loop {
            let (rank, parent) = self.forest.get(&current).expect(&no_parent_err!(current));

            // ja atradām root, tad laužam ciklu un atgriežam vērtību
            if current == *parent {
                break (*rank, current);
            }
            current = *parent;
        };

        return root;
    }

    pub fn union(&mut self, u: u32, v: u32) {
        let (u_rank, u_root) = self.find_set(u);
        let (v_rank, v_root) = self.find_set(v);

        if u_root == v_root {
            return;
        }

        if u_rank > v_rank {
            let (_rank, v_parent) = self.forest.get_mut(&v_root).expect(&no_parent_err!(v_root));
            *v_parent = u_root;
        } else {
            let (_rank, u_parent) = self.forest.get_mut(&u_root).expect(&no_parent_err!(u_root));
            *u_parent = v_root;

            if u_rank == v_rank {
                let (v_rank, _parent) =
                    self.forest.get_mut(&v_root).expect(&no_parent_err!(v_root));

                *v_rank += 1;
            }
        }
    }
}

// patur tikai tās šķautnes kuras ir grafā a
// apstrādā grafa šķautnes kā kopas a, b
// izpilda kopu set diff darbību a\b
pub fn graph_edge_set_diff(graph_a: &Graph, graph_b: &Graph) -> Graph {
    let mut complement = Graph::new();

    // hashset optimizācija, lai nav jāiterē cauri katrai a grafa virsotnei ar katru b grafa virsotni
    let b_edges: HashSet<Edge> = graph_b.edge_list.iter().map(|e| e.normalized()).collect();

    for edge_a in &graph_a.edge_list {
        if !b_edges.contains(&edge_a.normalized()) {
            complement.add_edge(*edge_a);
        }
    }

    complement
}

pub fn combine_graphs(graph_a: &mut Graph, graph_b: &Graph) {
    for edge_b in &graph_b.edge_list {
        graph_a.add_edge(*edge_b);
    }
}
