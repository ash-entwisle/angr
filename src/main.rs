use std::collections::HashMap;
use clap::{arg, Command};

fn cmd() -> Command {
    Command::new("angr")
        .about("A command line tool to to count the frequency of bigrams in a text file")
        .arg(arg!(file: -f --file <FILE> "The input file to read from"))
        .arg(arg!(n: -n --ngram <N> "The size of the n-gram to count"))
        .arg(arg!(output: -o --output <OUTPUT> "The output file to write to"))
        .arg(arg!(silent: -s --silent "Silence all output"))
        .arg(arg!(spaces: -S --spaces "Count spaces as characters"))
        .arg(arg!(reverses: -R --reverses "Count reverse n-grams as the same n-gram"))
}

fn main() {

    let matches = cmd().get_matches();
    let blank: String = "".to_string();

    let file: &String   = matches.get_one::<String>("file").unwrap();
    let n: &String      = matches.get_one::<String>("n").unwrap_or(&blank);
    let output: &String = matches.get_one::<String>("output").unwrap_or(&blank);
    let silent: &bool   = matches.get_one::<bool>("silent").unwrap_or(&false);
    let spaces: &bool   = matches.get_one::<bool>("spaces").unwrap_or(&false);
    let reverses: &bool = matches.get_one::<bool>("reverses").unwrap_or(&false);

    let text = std::fs::read_to_string(file).unwrap();
    let n: &usize = &n.parse().unwrap_or(2);

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

    if *reverses {
        let temp_counts = counts.clone();

        for (ngram, count) in temp_counts {

            let reversed_ngram = ngram.chars().rev().collect::<String>();

            if counts.contains_key(&reversed_ngram) {
            
                let reversed_count = counts.iter()
                    .find(|(tempgram, _)| tempgram == &&reversed_ngram)
                    .unwrap().1;

                counts.insert(ngram, count + reversed_count);
                counts.remove(&reversed_ngram);
            }
        }
    }

    // sort the counts
    let mut counts = counts
        .into_iter()
        .filter(|(ngram, _)| ngram.len() == *n && !spaces)
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
