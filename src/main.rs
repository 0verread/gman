use clap::Parser;
use rand::thread_rng;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Parser)]
struct Cli{
    #[arg(long)]
    file: PathBuf,
}

fn build_mc(words: &[String]) -> HashMap<String, Vec<String>> {
    let mut mc = HashMap::new();
    for word in words {
        for (i, window) in word.chars().collect::<Vec<_>>().windows(2).enumerate() {
            let prefix = window.iter().collect::<String>();
            let suffix = word.chars().skip(i + 2).collect::<String>();
            mc.entry(prefix).or_insert_with(Vec::new).push(suffix);
        }
    }
    return mc;
}

fn generate_name(mc: &HashMap<String, Vec<String>>, max_len: usize) -> String {
    let mut rng = thread_rng();
    let mut keys: Vec<_> = mc.keys().cloned().collect();
    let mut state = keys.choose(&mut rng).unwrap().to_string();
    let mut name = state.clone();

    while name.len() < max_len {
        let next_words = mc.get(&state).unwrap();
        let next_word = next_words.choose(&mut rng).unwrap();
        name.push_str(next_word);
        state = next_word.clone();
    }
    return name;
}

fn read_words_from_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut word_iter = line.split_whitespace();

        while let Some(word) = word_iter.next() {
            words.push(word.to_string());
        }
    }

    Ok(words)
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file.to_str().unwrap();
    let words = read_words_from_file(file_path).expect("Failed to read words from file");
    let mc = build_mc(&words);
    let name = generate_name(&mc, 3);
    println!("Name: {:?}", name);
}

