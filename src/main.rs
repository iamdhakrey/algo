use clap::{Arg, ArgAction, Command};

use crate::{
    generate::search_list,
    search::{SeachTrait, linear::LinearSearch},
};

mod generate;
mod search;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     /// Turn debugging information on
//     #[arg(short, long, action = clap::ArgAction::Count)]
//     debug: u8,

// }

fn main() {
    let matches = Command::new("algo")
        .about("Have Awesome Algorithms")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("search")
                .arg(
                    Arg::new("ALGONAME")
                        .help(
                            "The name of the algorithm to search for (e.g., quicksort, dijkstra).",
                        )
                        .required(true),
                )
                .arg(
                    Arg::new("debug")
                        .long("debug")
                        .help("Optional flag to request a detailed explanation.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", search_matches)) => {
            let algoname = search_matches
                .get_one::<String>("ALGONAME")
                .expect("ALGONAME is required");
            let detailed = search_matches.get_flag("debug");

            println!("🔍 Executing search command...");
            println!("Algorithm Name: {}", algoname);
            let search_list = search_list();
            println!("List: {:?}", search_list);
            if detailed {
                println!("Request Type: Detailed explanation requested.");
                println!("--------------------------------------------------");
                match algoname.to_lowercase().as_str() {
                    "linear" => {
                        println!(
                            "The linear search algorithm, also known as sequential search, is a straightforward method for finding a specific element within a list or array. It operates by sequentially examining each element of the list, one by one, until either the target element is found or the end of the list is reached"
                        );
                        let l = LinearSearch {
                            list_of_numbers: search_list,
                            seach_number: 101,
                        };
                        l.search();
                        l.print();
                    }
                    "quicksort" => println!(
                        "QuickSort is an efficient, comparison-based, in-place sorting algorithm. It employs a divide-and-conquer strategy, selecting a 'pivot' element and partitioning the other elements into two sub-arrays, according to whether they are less than or greater than the pivot. This process is then recursively applied to the sub-arrays. Its average time complexity is O(n log n)."
                    ),
                    "dijkstra" => println!(
                        "Dijkstra's Algorithm is an algorithm for finding the shortest paths between nodes in a graph, which may represent road networks. It works by visiting nodes in order of increasing distance from the source, updating the shortest path estimates for all neighbors as it goes. It requires the graph edges to have non-negative weights."
                    ),
                    _ => println!(
                        "No detailed information found for '{}'.d {}",
                        algoname,
                        algoname.to_lowercase().as_str()
                    ),
                }
            } else {
                println!("Request Type: Basic lookup.");

                match algoname.to_lowercase().as_str() {
                    "linear" => {
                        let l = LinearSearch {
                            list_of_numbers: search_list,
                            seach_number: 9,
                        };
                        l.search();
                        l.print();
                    }
                    _ => {
                        println!("Not added yet")
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}
