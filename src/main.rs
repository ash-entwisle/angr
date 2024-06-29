use std::collections::HashMap;

use clap::{arg, Command};

fn cmd() -> Command {
    Command::new("angr")
        .about("A command line tool to to count the frequency of bigrams in a text file")
        .arg(arg!(file: <FILE> "The input file to read from"))
        .arg(arg!(n: -n --ngram <N> "The size of the n-gram to count").default_value("2"))
        .arg(arg!(output: -o --output <OUTPUT> "The output file to write to"))
        .arg(arg!(silent: -s --silent "Silence all output"))
}

fn main() {

    let matches = cmd().get_matches();
    let blank: String = "".to_string();

    let file: &String   = matches.get_one::<String>("file").unwrap();
    let n: &u128        = matches.get_one::<u128>("n").unwrap_or(&2);
    let output: &String = matches.get_one::<String>("output").unwrap_or(&blank);
    let silent: &bool   = matches.get_one::<bool>("silent").unwrap_or(&false);

    let text = std::fs::read_to_string(file).unwrap();

    // for each word, for each character, for each ngram, count it
    let mut counts: HashMap<String, usize> = std::collections::HashMap::new();

    for word in text.split_whitespace() {
        let word = word.chars().collect::<Vec<char>>();
        for i in 0..word.len() {
            let mut ngram = String::new();
            for j in 0..*n {
                if (i + j as usize) < word.len() {
                    ngram.push(word[i + j as usize]);
                }
            }
            let count = counts.entry(ngram).or_insert(0);
            *count += 1;
        }
    }


    // sort the counts
    let mut counts = counts
        .into_iter()
        .filter(|(ngram, _)| ngram.len() == *n as usize)
        .collect::<Vec<(String, usize)>>();

    counts.sort_by(|a, b| b.1.cmp(&a.1));


    // print the counts
    if !silent {
        for (ngram, count) in &counts {
            println!("{}: {}", ngram, count);
        }
    }

    // write the counts to a file
    if output != &blank {
        let json = serde_json::to_string(&counts).unwrap();
        std::fs::write(output, json).unwrap();
    }
}
