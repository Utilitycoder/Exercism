use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let mut workers = Vec::new();
    for _ in 0..worker_count {
        workers.push(std::thread::spawn(|| HashMap::new()));
    }
    for chunk in input.chunks(worker_count) {
        for (i, line) in chunk.iter().enumerate() {
            let worker = &mut workers[i];
            let worker_result = worker.join().unwrap();
            for c in line.chars().filter(|c| c.is_alphabetic()) {
                *worker_result.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            }
            *worker = std::thread::spawn(|| HashMap::new());
        }
    }
    for worker in workers {
        let worker_result = worker.join().unwrap();
        for (c, count) in worker_result {
            *result.entry(c).or_insert(0) += count;
        }
    }
    result
}
