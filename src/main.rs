mod parser;

use self::parser::Parser;
use std::{cmp, collections::HashMap, vec};

#[derive(Eq, PartialOrd, Clone, Copy, Debug)]
pub struct Edge {
    u: u8,
    v: u8,
    w: i8,
}

// šķautne ir tā pati neatkarībā vai salīdzinām (u,v,w) vai (v,u,w)
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        return (self.u == other.u && self.v == other.v && self.w == other.w)
            || (self.u == other.v && self.v == other.u && self.w == other.w);
    }
}

// edge instances get compared (for sorting purposes) by weight
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        return self.w.cmp(&other.w);
    }
}

#[derive(Debug)]
pub struct Graph {
    edge_list: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edge_list: vec![] }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        assert!(!self.edge_list.contains(&edge));

        self.edge_list.push(edge);
    }

    pub fn sort_edge_list(&mut self) {
        self.edge_list.sort();
    }
}

pub struct UnionFind {
    // key - virosotnes id,
    // value - parent virsotnes id
    // root virsotnes parent ir viņa pati
    parent: HashMap<u8, u8>,
}

impl UnionFind {
    pub fn new() -> UnionFind {
        UnionFind {
            parent: HashMap::new(),
        }
    }

    // ievieto virsotni savā kopā - tās parent ir viņa pati
    pub fn make_set(&mut self, v: u8) {
        self.parent.insert(v, v);
    }

    // atrod virsotnes v root virsotni
    // principā tiek atrasta kādas virsotņu kopas root virsotne
    pub fn find(&self, v: u8) -> u8 {
        let v_parent = self.parent.get(&v);

        match v_parent {
            Some(vp) => {
                if *vp == v {
                    return v;
                }

                return self.find(*vp);
            }
            None => panic!("Value {v} does not have a parent!"),
        }
    }

    // apvieno kopu kurā atrodas virsotne a ar kopu kurā atrodas virsotne b,
    // a virsotnes root virsotnes parent tiek iestatīts kā b virsotnes root
    pub fn union(&mut self, a: u8, b: u8) {
        let a_root = self.find(a);
        let b_root = self.find(b);

        assert!(a_root != b_root);

        self.parent.insert(a_root, b_root);
    }
}

// ievades dati formātā:
// n a_1 b_1 w_1 a_2 b_2 w_2 ... a_m b_m w_m,
//
// n - virsotņu skaits grafā (n < 5000)
// a_i, b_i iekš {1, ..., n}
// w_i iekš {-99, ... 99}
// a_i b_i w_1 reprezentē šķautni starp virsotnēm a,b, ar svaru w
// šķautni var reprezetnēt gan kā a_i, b_i, w_i, gan b_i, a_i, w_i
fn parse_input(input: String) -> Graph {
    let mut parser = Parser::new(input);

    let mut graph = Graph::new();

    parser.consume_whitespace();

    let n = parser
        .next_while(|c| !c.is_whitespace())
        .parse::<i32>()
        .unwrap();

    assert!(n > 0 && n < 5000);

    parser.consume_whitespace();

    while !parser.eof() {
        let a = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u8>()
            .unwrap();
        assert!(a >= 1 && i32::from(a) <= n);

        parser.consume_whitespace();
        let b = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u8>()
            .unwrap();
        assert!(b >= 1 && i32::from(b) <= n);

        parser.consume_whitespace();
        let w = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<i8>()
            .unwrap();

        assert!(w >= -99 && w <= 99);

        // datu konsekvences dēļ - virsotne ar mazāko id tiks ievietota šķautnē pirmā
        if a < b {
            graph.add_edge(Edge { u: a, v: b, w });
        } else {
            graph.add_edge(Edge { u: b, v: a, w });
        }

        parser.consume_whitespace();
    }

    return graph;
}

// max_weight_span_tree_kruskal
fn mst_kruskal(mut graph: Graph) -> Graph {
    graph.edge_list.sort();

    let mut union_find = UnionFind::new();
    let mut a = Graph::new();

    for edge in &graph.edge_list {
        if !union_find.parent.contains_key(&edge.u) {
            union_find.make_set(edge.u);
        }

        if !union_find.parent.contains_key(&edge.v) {
            union_find.make_set(edge.v);
        }
    }

    for edge in &graph.edge_list {
        if union_find.find(edge.u) != union_find.find(edge.v) {
            a.add_edge(edge.clone());
            union_find.union(edge.u, edge.v);
        }
    }

    return a;
}

fn main() {
    // katrai derīgā cikliskā maršrutā no virsotnes v (līdz ar to atgriežamies virsontē v),
    // ir vismaz viens viena šķautni ar medus podu

    // derīgs ciklisks maršruts ir tāds, kurā katra šķautne ir dažāda, tās neatkārtojas

    // grūtības pakāpe jāoptimizē - no visiem iespējamajiem krājumu izvietojumiem jāizvēlas mazākā iespējāmā svaru summa

    // potential solution:
    // max weight spanning tree ar Kruskals algorithm
    // šķautnes, kuras nav iekšā šajā kokāgraph ir mums meklējāmas
    // optimizācija - noņem virsotnes iteratīvi iekš mst_kruskal funkcijas no īstā grafa

    // let parsed_graph = parse_input("2 1 2 -4".to_string());
    let parsed_graph = parse_input("5 1 2 -1 ".to_string());
    println!("{:#?}", parsed_graph);

    let max_span_tree = mst_kruskal(parsed_graph);
    println!("{:#?}", max_span_tree);
}
