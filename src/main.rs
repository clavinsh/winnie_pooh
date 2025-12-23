mod parser;

use self::parser::Parser;
use std::collections::HashMap;

// lai gan HashMap struktūrā var teorētiski gadīties kolīzija, strādājam ar relatīvi mazu skaitu
// elementiem, kuriem katra vērtība ir unikāla, praktiski runājot, nolāsīšana un ievietošana ir ar
// sarežģītību O(1)
#[derive(Debug)]
pub struct Graph {
    adj_list: HashMap<u8, HashMap<u8, i8>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: u8, b: u8, w: i8) {
        self.adj_list.entry(a).or_insert_with(HashMap::new);
        self.adj_list.entry(b).or_insert_with(HashMap::new);

        self.adj_list.get_mut(&a).unwrap().insert(b, w);
        self.adj_list.get_mut(&b).unwrap().insert(a, w);
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

    for _ in 0..n {
        parser.consume_whitespace();
        let a = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u8>()
            .unwrap();
        parser.consume_whitespace();
        let b = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<u8>()
            .unwrap();
        parser.consume_whitespace();
        let w = parser
            .next_while(|c| !c.is_whitespace())
            .parse::<i8>()
            .unwrap();

        graph.add_edge(a, b, w);
    }

    graph
}


fn main() {
    let parsed_input = parse_input("1 1 2 -4".to_string());


    // katrai cikliskai 

    println!("{:#?}", parsed_input);
}
