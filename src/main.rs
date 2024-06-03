use std::{
    collections::HashMap,
    f32::INFINITY,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let datapath = "../data/measurements.txt";

    let file = File::open(datapath).expect("Panicked while opening file");
    let reader = BufReader::new(file);

    //let chunk_size = 1_000_000;
    let mut data: HashMap<&str, Vec<f32>> = HashMap::new();

    for line in reader.lines() {
        let row = line.expect("Panicked while loading record");
        let record: Vec<&str> = row.split(';').collect();
        let city = record[0].clone();
        let temperature: f32 = match record[1].clone().parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };

        let corr_entry = data.entry(city).or_insert(vec![INFINITY, 0.0, 0.0, 0.0]);

        corr_entry[0] = if temperature < corr_entry[0] {
            temperature
        } else {
            corr_entry[0]
        };
        corr_entry[1] += ((corr_entry[1] * corr_entry[3]) + temperature) / (corr_entry[3] + 1.0);
        corr_entry[2] = if temperature > corr_entry[2] {
            temperature
        } else {
            corr_entry[2]
        };
        corr_entry[3] += 1.0;
    }

    let outfile = File::create("output.txt").expect("Panicked while opening/creating outfile");
    let mut writer = BufWriter::new(outfile);

    for (city, stats) in &data {
        let outline = format!("{}={}/{}/{}, ", city, stats[0], stats[1], stats[2]);
        writer
            .write_all(outline.as_bytes())
            .expect("Panicked while writing outline");
    }

    writer.flush().expect("Panicked while flushing the output");
}
