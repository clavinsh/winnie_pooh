mod parser;

use rand::seq::SliceRandom;

use self::parser::Parser;
use std::{collections::HashMap, fmt::Write, fs, vec};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Edge {
    u: u32,
    v: u32,
    w: i8,
}

impl Edge {
    pub fn is_equal(&self, other: &Self) -> bool {
        (self.u == other.u && self.v == other.v && self.w == other.w)
            || self.u == other.v && self.v == other.u && self.w == other.w
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
        self.edge_list.push(edge);
    }

    pub fn sort_by_weight_desc(&mut self) {
        self.edge_list.sort_by_key(|e| std::cmp::Reverse(e.w));
    }

    pub fn invert_edge_weights(&mut self) {
        for edge in &mut self.edge_list {
            edge.w *= -1;
        }
    }

    pub fn sort_by_weight_asc(&mut self) {
        self.edge_list.sort_by_key(|e| e.w);
    }

    // for testing purpoes
    pub fn randomize(&mut self) {
        let mut rng = rand::rng();
        self.edge_list.shuffle(&mut rng);
    }
}

pub struct UnionFind {
    // key - virosotnes id,
    // value - parent virsotnes id
    // root virsotnes parent ir viņa pati
    parent: HashMap<u32, u32>,
}

impl UnionFind {
    pub fn new() -> UnionFind {
        UnionFind {
            parent: HashMap::new(),
        }
    }

    // ievieto virsotni savā kopā - tās parent ir viņa pati
    pub fn make_set(&mut self, v: u32) {
        self.parent.insert(v, v);
    }

    // atrod virsotnes v root virsotni
    // principā tiek atrasta kādas virsotņu kopas root virsotne
    pub fn find(&self, v: u32) -> u32 {
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
    pub fn union(&mut self, a: u32, b: u32) {
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
fn parse_input(input: String) -> (Graph, u32) {
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
        assert!(a >= 1 && a <= n.try_into().unwrap());

        parser.consume_whitespace();
        let b = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u32>()
            .unwrap();
        assert!(b >= 1 && b <= n.try_into().unwrap());

        parser.consume_whitespace();
        let w = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<i8>()
            .unwrap();

        assert!(w >= -99 && w <= 99);

        graph.add_edge(Edge { u: a, v: b, w });

        parser.consume_whitespace();
    }

    return (graph, n);
}

// max_weight_span_tree_kruskal
// grafam obligāti jau jābūt sakārtotam
fn mst_kruskal(graph: &Graph) -> Graph {
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

// patur tikai tās šķautnes kuras ir grafā a
// apstrādā grafa šķautnes kā kopas a, b
// izpilda kopu set diff darbību a\b
pub fn graph_edge_set_diff(graph_a: &Graph, graph_b: &Graph) -> Graph {
    let mut complement = Graph::new();

    for edge_a in &graph_a.edge_list {
        if !graph_b.edge_list.iter().any(|e| e.is_equal(edge_a)) {
            complement.add_edge(*edge_a);
        }
    }

    complement
}

pub fn graph_weight_sum(graph: &Graph) -> i32 {
    let mut sum: i32 = 0;
    for edge in &graph.edge_list {
        sum += i32::from(edge.w);
    }
    return sum;
}

pub fn serialize_honey_edges(graph: &Graph) -> String {
    let mut serialized = String::new();

    let k = graph.edge_list.len();
    let w = graph_weight_sum(graph);

    writeln!(serialized, "{} {}", k, w).expect("Failed to serialize graph to a string");

    for edge in &graph.edge_list {
        writeln!(serialized, "{} {}", edge.u, edge.v)
            .expect("Failed to serialize graph to a string");
    }

    return serialized;
}

pub fn to_dot_fmt(graph: &Graph, vertex_count: u32) -> String {
    let mut dot = String::from(
        "graph G {\nlayout=neato;\noverlap=scale;\nsplines=true;\nsep=\"+15\";\n\nnode [shape=circle, width=0.2];\nedge [fontsize=8];\n\n",
    );

    dot.push_str("  node [shape=circle];\n");

    for i in 1..=vertex_count {
        dot.push_str(&format!("   {};\n", i));
    }

    for edge in &graph.edge_list {
        dot.push_str(&format!(
            "   {} -- {} [xlabel=\"{}\"];\n",
            edge.u, edge.v, edge.w
        ));
    }

    dot.push_str("}\n");

    return dot;
}

fn main() -> Result<(), std::io::Error> {
    // katrai derīgā cikliskā maršrutā no virsotnes v (līdz ar to atgriežamies virsontē v),
    // ir vismaz viens viena šķautni ar medus podu

    // derīgs ciklisks maršruts ir tāds, kurā katra šķautne ir dažāda, tās neatkārtojas

    // grūtības pakāpe jāoptimizē - no visiem iespējamajiem krājumu izvietojumiem jāizvēlas mazākā iespējāmā svaru summa

    // potential solution:

    // min weight spanning tree ir grafa šķautnes, kuras:
    //      savieno visas virstones
    //      neveido ciklus
    //      ir ar vismazākajiem svariem
    //
    // ja izdosies atrast pretējo - max weight spanning tree, tad:
    //      atlikušās šķautnes būs tās, kuras savienojot, izveidos ciklus (medus poda prasība)
    //      tām būs pēc iespējas mazāka vērtība, jo lielākās vērtības būs iekš spanning tree

    // max weight spanning tree ar Kruskals algorithm
    // šķautnes, kuras nav iekšā šajā kokā ir mums meklējāmas
    // optimizācija - noņem virsotnes iteratīvi iekš mst_kruskal funkcijas no īstā grafa

    let file_contents_result = fs::read_to_string(
        "/home/artursk/magistrs/efficient_algos/winnie_pooh/text_samples/sample_input_2025_3.txt",
    );

    match file_contents_result {
        Ok(content) => {
            let (mut parsed_graph, n) = parse_input(content);

            parsed_graph.randomize();

            parsed_graph.invert_edge_weights();
            parsed_graph.sort_by_weight_asc();

            let mst = mst_kruskal(&parsed_graph);
            let honey_edges = graph_edge_set_diff(&parsed_graph, &mst);

            let result = serialize_honey_edges(&honey_edges);

            let dot_format = to_dot_fmt(&parsed_graph, n);

            fs::write(
                "/home/artursk/magistrs/efficient_algos/winnie_pooh/output/result.txt",
                result,
            )?;

            fs::write(
                "/home/artursk/magistrs/efficient_algos/winnie_pooh/output/graph_result.dot",
                dot_format,
            )?;
        }
        Err(e) => return Err(e),
    }

    return Ok(());
}
