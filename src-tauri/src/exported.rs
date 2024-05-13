use rand::distributions::Distribution;
use rand::{distributions::WeightedIndex, thread_rng, Rng};
use serde_json::json;

fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    let chars: Vec<char> = (0..length)
        .map(|_| rng.gen_range(b'a'..b'z') as char)
        .collect();
    chars.iter().collect()
}

#[derive(Debug)]
struct MyData {
    id: u64,
    name: String,
}

fn generate_random_data() -> MyData {
    let mut rng = thread_rng();
    MyData {
        id: rng.gen(),
        name: generate_random_string(5),
    }
}

#[tauri::command]
pub fn create_json_string() -> String {
    let data = generate_random_data();
    let weights = [8, 2];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let is_last = dist.sample(&mut rng) == 1;

    let json_obj = json!({
        "id": data.id,
        "name": &data.name,
        "is_last": is_last,
    });

    // json_obj["is_last"] = json!(is_last);
    json_obj.to_string()
}
