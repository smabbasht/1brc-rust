use std::{
    collections::HashMap,
    f32::INFINITY,
    fs::File,
    io::{BufWriter, Read, Write},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let datapath = "data/measurements.txt";
    let mut file = File::open(datapath).expect("Panicked while opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Panicked while reading file");
    println!("Done reading the file");

    let mut data: HashMap<String, [f32; 4]> = HashMap::new();

    let mut counter = 0;
    for line in contents.lines() {
        if let Some((state, temperature)) = line.split_once(';') {
            let temperature: f32 = temperature.trim().parse().unwrap_or(0.0);
            let entry = data
                .entry(state.to_string())
                .or_insert([INFINITY, 0.0, 0.0, 0.0]);
            entry[0] = entry[0].min(temperature);
            entry[1] = (entry[1] * entry[3] + temperature) / (entry[3] + 1.0);
            entry[2] = entry[2].max(temperature);
            entry[3] += 1.0;
        }
        counter += 1;
        if counter == 1_000_000 {
            println!("{}", counter);
        }
    }

    let outfile = File::create("output.txt").expect("Panicked while opening/creating outfile");
    let mut writer = BufWriter::new(outfile);

    for (state, stats) in &data {
        writeln!(writer, "{}={}/{}/{}", state, stats[0], stats[1], stats[2])
            .expect("Panicked while writing outline");
    }

    writer.flush().expect("Panicked while flushing the output");
    let elapsed_time = now.elapsed();
    println!("The code took {} seconds to run", elapsed_time.as_secs());
}
