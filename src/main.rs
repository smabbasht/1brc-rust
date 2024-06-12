use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let n_threads = available_parallelism().unwrap().get();

    // let datalines = fs::read_to_string("data/measurements_lite.txt").unwrap();
    let datalines = fs::read_to_string("data/measurements.txt").unwrap();
    let lines: Vec<&str> = datalines.lines().collect();

    let n_lines = lines.len();

    let chunk_size = (n_lines) / n_threads;

    let data_map: Arc<Mutex<HashMap<&str, [f32; 4]>>> = Arc::new(Mutex::new(HashMap::new()));
    println!(
        "Done doing init and readfile in {}s",
        now.elapsed().as_secs()
    );

    let mut counter = 0;
    std::thread::scope(|s| {
        for chunk in lines.chunks(chunk_size) {
            counter += 1;
            let data_map = Arc::clone(&data_map);
            s.spawn(move || {
                let now = Instant::now();
                println!("Thread {} started", counter);
                let chunk = chunk.to_vec();
                let mut local_chunk: HashMap<&str, [f32; 4]> = HashMap::new();

                for line in chunk {
                    if let Some((state, temperature)) = line.split_once(';') {
                        let temperature = temperature.parse().unwrap();
                        let entry = local_chunk
                            .entry(state)
                            .or_insert([100.0, 0.0, -100.0, 0.0]);
                        entry[0] = entry[0].min(temperature);
                        entry[1] += temperature;
                        entry[2] = entry[2].max(temperature);
                        entry[3] += 1.0;
                    }
                }

                let data = &mut data_map.lock().unwrap();
                for (state, stats) in local_chunk {
                    let entry = data.entry(state).or_insert([100.0, 0.0, -100.0, 0.0]);
                    entry[0] = stats[0].min(entry[0]);
                    entry[1] += stats[1];
                    entry[2] = stats[2].max(entry[2]);
                    entry[3] += stats[3];
                }
                println!(
                    "Thread {} is ending in {}ms",
                    counter,
                    now.elapsed().as_millis()
                );
            });
        }
    });

    let now = Instant::now();
    let outfile = File::create("output.txt").expect("Panicked while opening/creating outfile");
    let mut writer = BufWriter::new(outfile);
    let data = data_map.lock().unwrap();

    for (state, stats) in &*data {
        let outline = format!(
            "{}={}/{:.1}/{}, ",
            state,
            stats[0],
            stats[1] / stats[3],
            stats[2]
        );
        writer
            .write_all(outline.as_bytes())
            .expect("Panicked while writing outline");
    }

    writer.flush().expect("Panicked while flushing the output");
    println!("Done writing to file in {}ms", now.elapsed().as_millis());
}
