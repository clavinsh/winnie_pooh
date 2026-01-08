mod parser;
mod winnie_pooh;

use std::io::Error;
use std::{env, fs};

use self::winnie_pooh::*;
fn print_cli_help() {
    println!(
        "Usage:
    winnie_pooh <input_file> <output_file>"
    );
}

fn winnie_pooh(input_file_path: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    let input_result = fs::read_to_string(input_file_path);

    match input_result {
        Ok(input) => {
            // ievadfaila struktūra, galvenokārt, apraksta šķautnes
            // parseris izveido grafa objektu vienu reizi caurstaigājot ievadfailu
            // O(|E|)
            let mut parsed_graph = Graph::from_input(input);

            //  šķautnu sakārtošana un mst_kruskal kopā veido sarežģītību O(|E| * log |V|)
            parsed_graph.sort_by_weight_desc();
            let mst = parsed_graph.mst_kruskal();

            // pēc MST definīcijas, rezultātā iegūtais šķautņu skaits mst kokā ir |V| - 1,
            // tāpēc tālāk vietās, kur tiek apskatītās mst šķautnes,
            // no sākotnējā grafa tās būs skaitā O(|V|)

            // šķautņu hashset izveide mst grafam ir O(|V|)
            // šķautņu pārbaude sākotnējam grafam ir O(|E|),
            // kopā sanāk O(|V| + |E|), bet tā kā grafs ir connected, tad:
            //      |E| >= |V| - 1
            //      O(|V| + |E|) = O(|E|)
            let mut honey_edges = graph_edge_set_diff(&parsed_graph, &mst);

            // bez atrastajām šķautnēm caur sākotnējā grafa starpību ar max spanning tree,
            // kuras atrod visas šķautnes, lai katrā ciklā būtu medus pods,
            // iespējams samazināt kopējo svaru summu izvēlotes vēl klāt šķautnes,
            // kuru vērtība ir negatīva,
            //
            // tiek apskatītas MST šķautnes - O(|V|)
            for edge in &mst.edge_list {
                if edge.w < 0 {
                    honey_edges.add_edge(*edge);
                }
            }

            // grafa serializēšana sastāv no:
            //      šķautnu skaita aprēķināšanas - O(|E|)
            //      šķautnu svaru summas aprēķināšanas - O(|E|)
            //      katras šķautnes attiecīgo virsotņu pāra izvadīšana failā - O(|E|)
            // līdz ar to kopsummā - O(|E|)
            let output = honey_edges.serialize_edges();

            fs::write(output_file_path, output)?;

            // līdz ar to viss algoritms kopā - O(|E| * log |V|)
        }
        Err(e) => return Err(e),
    }

    return Ok(());
}

fn main() -> Result<(), std::io::Error> {
    // katrai derīgā cikliskā maršrutā no virsotnes v (līdz ar to atgriežamies virsontē v),
    // ir vismaz viena šķautne ar medus podu

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

    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let input_file_path = &args[1];
            let output_file_path = &args[2];

            return winnie_pooh(input_file_path, output_file_path);
        }
        _ => {
            eprintln!("Expected 2 CLI arguments, got {}", args.len() - 1);
            print_cli_help();
            let err_msg = Error::new(std::io::ErrorKind::InvalidInput, "Invalid CLI arguments");
            return Err(err_msg);
        }
    }
}
