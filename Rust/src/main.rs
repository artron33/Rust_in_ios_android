use std::fs;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let depth = 3;
    let all_words = read_files_into_word_array(&["src/hgwells.txt", "src/franklin.txt", "src/dickens.txt"]);
    let map = build_map(&all_words, depth);

    let initial_phrase = "and then I".to_string();
    let sentence = create_sentence(&initial_phrase, &map, 50, depth);
    println!();
    println!();
    println!("{}: {}", initial_phrase, sentence);
}

pub fn start(input: String, path1: String, path2: String, path3: String) -> String {
    let depth = 3;
    //let all_words = read_files_into_word_array(&[&to, "Assets/franklin.txt", "Assets/dickens.txt"]);
    let all_words = read_files_into_word_array(&[&path1, &path2, &path3]);
    let map = build_map(&all_words, depth);

    let initial_phrase = input.to_string();// "and then I".to_string();
    let sentence = create_sentence(&initial_phrase, &map, 50, depth);
    println!();
    println!();
    println!("{}: {}", initial_phrase, sentence);

    sentence
}

// Creates a new sequence of words, of max length, given a starting phrase
fn create_sentence(start_phrase: &String, word_map: &std::collections::HashMap<String, std::collections::HashMap<String, u32>>, sentence_length: usize, depth: usize) -> String {
    let mut sentence = String::new();
    let mut current_phrase = start_phrase.to_string();

    for _ in 0..sentence_length {
        if let Some(word_list) = word_map.get(&current_phrase) {
            let next_word = choose_word_weighted(word_list);
            sentence.push_str(&format!("{} ", next_word));

            let tokenized_phrase: Vec<&str> = current_phrase.split_whitespace().collect();
            current_phrase = tokenized_phrase.iter().skip(1).take(depth - 1).cloned().collect::<Vec<&str>>().join(" ");
            current_phrase.push_str(&format!(" {}", next_word));
        }
    }

    sentence
}

fn suggest_word(start_phrase: &str, word_map: &std::collections::HashMap<String, std::collections::HashMap<String, u32>>) -> String {
    if let Some(word_list) = word_map.get(start_phrase) {
        choose_word_weighted(word_list)
    } else {
        String::new()
    }
}

fn choose_word(word_list: &std::collections::HashMap<String, u32>) -> String {
    let keys: Vec<&String> = word_list.keys().collect();
    let random_index = (keys.len() as f64 * rand::random::<f64>()) as usize;
    keys[random_index].to_string()
}

fn choose_word_weighted(word_list: &std::collections::HashMap<String, u32>) -> String {
    let sum_of_weights: u32 = word_list.values().sum();
    let mut random = rand::random::<u32>() % sum_of_weights;

    let mut curr_word = String::new();
    for (word, &weight) in word_list.iter() {
        curr_word = word.clone();
        random = random.saturating_sub(weight);
        if random == 0 {
            break;
        }
    }

    curr_word
}

/*
   Runs through the list, gets the next n-1 words, and maps it to the n+1 word
*/
fn build_map(tokens: &[String], depth: usize) -> std::collections::HashMap<String, std::collections::HashMap<String, u32>> {
    let mut map = std::collections::HashMap::new();

    for index in 0..tokens.len().saturating_sub(depth) {
        let phrase: String = tokens[index..index + depth].join(" ");
        let phrase = phrase.trim_start().to_string();

        if !map.contains_key(&phrase) {
            map.insert(phrase.clone(), std::collections::HashMap::new());
        }

        let next_word = tokens[index + depth].clone();
        let next_word_list = map.get_mut(&phrase).unwrap();

        *next_word_list.entry(next_word).or_insert(0) += 1;
    }

    map
}

fn read_files_into_word_array(filenames: &[&str]) -> Vec<String> {
    let mut data = String::new();

    for filename in filenames {
        println!("> > > >");
        println!("> > > >");
        println!("> > > >file reading is: {}", filename);
        data.push_str(&format!(" {}", fs::read_to_string(filename).unwrap()));
    }

    // remove newlines
    data = data.replace("\r\n", " ");
    data = data.replace("\n", " ");

    // Put spaces around each special character / punctuation,
    // so that when we split on spaces, they come out as their own tokens,
    // disconnected from surrounding words
    let replacements = [',', '.', ':', '!', '?', '"', '“', '”', ';', '(', ')', '-', '_'];
    for value in replacements.iter() {
        // data = data.replace(value, &format!(" {} ", value));
        data = data.replace(*value, &format!(" {} ", value));

    }

    // Split on spaces to get each word by itself, indexed.
    let mut word_array: Vec<String> = data.split_whitespace().map(|s| s.to_string()).collect();

    // remove all pure whitespace entries
    word_array.retain(|word| word.trim().len() != 0);

    word_array
}