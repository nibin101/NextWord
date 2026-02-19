use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    net::SocketAddr,
    sync::Arc,
};

#[repr(C)]
#[derive(Clone, Copy)]
struct Entry {
    w1: u32,
    w2: u32,
    w3: u32,
    count: u32,
}

#[derive(Clone)]
struct AppState {
    entries: Arc<Vec<Entry>>,
    vocab: Arc<Vec<String>>,
    word_to_id: Arc<HashMap<String, u32>>,
}

#[derive(Deserialize)]
struct PredictRequest {
    context: String,
}

#[derive(Serialize)]
struct PredictResponse {
    suggestions: Vec<String>,
}

fn load_model() -> std::io::Result<Vec<Entry>> {
    let mut file = File::open("model.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let entries: Vec<Entry> = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const Entry,
            buffer.len() / std::mem::size_of::<Entry>(),
        )
        .to_vec()
    };

    Ok(entries)
}

fn load_vocab() -> std::io::Result<Vec<String>> {
    let file = File::open("vocab.txt")?;
    let reader = BufReader::new(file);

    let mut vocab = Vec::new();
    for line in reader.lines() {
        vocab.push(line?);
    }

    Ok(vocab)
}

fn predict(
    entries: &[Entry],
    vocab: &[String],
    word_to_id: &HashMap<String, u32>,
    w1: &str,
    w2: &str,
) -> Vec<String> {
    let id1 = match word_to_id.get(w1) {
        Some(id) => *id,
        None => return vec![],
    };

    let id2 = match word_to_id.get(w2) {
        Some(id) => *id,
        None => return vec![],
    };

    let mut results = Vec::new();

    if let Ok(mut pos) =
        entries.binary_search_by_key(&(id1, id2), |e| (e.w1, e.w2))
    {
        while pos < entries.len()
            && entries[pos].w1 == id1
            && entries[pos].w2 == id2
        {
            let w3_id = entries[pos].w3;
            results.push(vocab[w3_id as usize].clone());
            pos += 1;
        }
    }

    results.truncate(5);
    results
}

async fn predict_handler(
    State(state): State<AppState>,
    Json(payload): Json<PredictRequest>,
) -> Json<PredictResponse> {
    let lower = payload.context.to_lowercase();
    let tokens: Vec<&str> = lower.split_whitespace().collect();

    if tokens.len() < 2 {
        return Json(PredictResponse {
            suggestions: vec![],
        });
    }

    let w1 = tokens[tokens.len() - 2];
    let w2 = tokens[tokens.len() - 1];

    let suggestions = predict(
        &state.entries,
        &state.vocab,
        &state.word_to_id,
        w1,
        w2,
    );

    Json(PredictResponse { suggestions })
}

#[tokio::main]
async fn main() {
    println!("Loading model...");
    let entries = load_model().expect("Failed to load model.bin");

    println!("Loading vocabulary...");
    let vocab = load_vocab().expect("Failed to load vocab.txt");

    println!("Rebuilding word_to_id map...");
    let mut word_to_id = HashMap::new();
    for (i, word) in vocab.iter().enumerate() {
        word_to_id.insert(word.clone(), i as u32);
    }

    let state = AppState {
        entries: Arc::new(entries),
        vocab: Arc::new(vocab),
        word_to_id: Arc::new(word_to_id),
    };

    let app = Router::new()
        .route("/predict", post(predict_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
