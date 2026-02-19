use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

#[repr(C)]
#[derive(Debug)]
struct Entry {
    w1: u32,
    w2: u32,
    w3: u32,
    count: u32,
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
}

fn get_id(
    word: &str,
    word_to_id: &mut HashMap<String, u32>,
    id_to_word: &mut Vec<String>,
) -> u32 {
    if let Some(&id) = word_to_id.get(word) {
        id
    } else {
        let id = id_to_word.len() as u32;
        word_to_id.insert(word.to_string(), id);
        id_to_word.push(word.to_string());
        id
    }
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    println!("Opening data file...");
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let mut word_to_id: HashMap<String, u32> = HashMap::new();
    let mut id_to_word: Vec<String> = Vec::new();

    let mut trigram: HashMap<(u32, u32), HashMap<u32, u32>> = HashMap::new();

    println!("Building trigram counts...");

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if line_number % 100_000 == 0 {
            println!("Processed {} lines", line_number);
        }

        let tokens = tokenize(&line);

        if tokens.len() < 3 {
            continue;
        }

        let ids: Vec<u32> = tokens
            .iter()
            .map(|w| get_id(w, &mut word_to_id, &mut id_to_word))
            .collect();

        for window in ids.windows(3) {
            let w1 = window[0];
            let w2 = window[1];
            let w3 = window[2];

            let entry = trigram
                .entry((w1, w2))
                .or_insert_with(HashMap::new);

            *entry.entry(w3).or_insert(0) += 1;
        }
    }

    println!("Pruning rare entries...");

    let min_count = 2; // increase later if model too large

    for (_, next_words) in trigram.iter_mut() {
        next_words.retain(|_, count| *count >= min_count);
    }

    println!("Flattening model...");

    let mut entries: Vec<Entry> = Vec::new();

    for ((w1, w2), next_words) in trigram {
        for (w3, count) in next_words {
            entries.push(Entry { w1, w2, w3, count });
        }
    }

    entries.sort_by_key(|e| (e.w1, e.w2));

    println!("Saving model.bin ...");

    let mut model_file = File::create("model.bin")?;

    let bytes = unsafe {
        std::slice::from_raw_parts(
            entries.as_ptr() as *const u8,
            entries.len() * std::mem::size_of::<Entry>(),
        )
    };

    model_file.write_all(bytes)?;

    println!("Saving vocab.txt ...");

    let mut vocab_file = File::create("vocab.txt")?;
    for word in id_to_word {
        writeln!(vocab_file, "{}", word)?;
    }

    println!("Done!");
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
