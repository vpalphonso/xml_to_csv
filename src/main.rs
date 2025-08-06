use std::{ fs, io::{ self }, thread };
use serde::Serialize;
use regex::Regex;

fn main() -> io::Result<()> {
    println!("Enter path for units to convert XML to CSV");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{:?}", buffer.trim());

    let paths = fs::read_dir(buffer.trim()).unwrap();
    let mut handles = vec![];
    for path in paths {
        let handle = thread::spawn(move || {
            find_children(path.unwrap().path().display().to_string());
        });

        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(result) => result,
            Err(e) => eprintln!("Thread failed: {:?}", e),
        }
    }

    Ok(())
}

fn find_children(path: String) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            find_children(path.display().to_string());
        }
        if path.is_file() {
            convert_xml_to_csv(path.display().to_string());
        }
    }
}

#[derive(Debug, Serialize)]
struct Record {
    time: String,
    value: String,
}

fn convert_xml_to_csv(path: String) {
    let contents = fs
        ::read_to_string(path.clone())
        .expect("Should have been able to read the file");
    let splitxml: Vec<_> = path.split(".xml").collect();
    if splitxml[0].contains(&".csv") {
        return;
    }
    let data = format!("{}.csv", splitxml[0]);
    println!("{:?}", splitxml);

    let re = Regex::new(
        r#"<POINT>\s*<TIME>(.*?)</TIME>\s*<VALUE>(.*?)</VALUE>\s*</POINT>"#
    ).unwrap();

    let mut csv_trends = vec![];
    for cap in re.captures_iter(&contents) {
        let time = cap.get(1).unwrap().as_str();
        let value = cap.get(2).unwrap().as_str();
        let data = Record {
            time: time.to_string(),
            value: value.to_string(),
        };
        csv_trends.push(data);
    }

    let mut csv_data = String::from("Time,Value\n");
    for r in csv_trends {
        csv_data.push_str(&format!("{},{}\n", r.time, r.value));
    }
    let _ = fs::write(&data, &csv_data);
}
