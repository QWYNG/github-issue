use serde_json::Value;
use std::borrow::Borrow;

pub fn print_as_table(rows: Value, headers: Vec<&str>) -> () {
    let header_copy = headers.clone();
    let row_copy = rows.clone();
    let columns = split_columns(rows, headers);
    let widths = widths_of(&columns);

    print_one_line(
        &header_copy.iter().map(|s| s.to_string()).collect(),
        &widths,
    );
    print_columns(columns, &widths);
}

fn split_columns(rows: Value, headers: Vec<&str>) -> Vec<Vec<String>> {
    let mut vec = Vec::new();
    for header in headers {
        let mut column = Vec::new();
        for row in rows.as_array().unwrap().to_owned() {
            if row[header].is_number() {
                column.push(row[header].as_u64().unwrap().to_string())
            } else {
                column.push(row[header].as_str().unwrap().to_string())
            }
        }
        vec.push(column);
    }
    vec
}

#[test]
fn split_columns_test() {
    let data = r#"
        [
            {"number": 1, "name": "Thresh",  "role": "Support"},
            {"number": 2, "name": "Lucian",  "role": "ADC"},
            {"number": 3, "name": "Shyvana", "role": "Fighter"}
        ]"#;

    let v: serde_json::Value = serde_json::from_str(data).unwrap();
    let columns = split_columns(v, vec!["number", "name", "role"]);
    println!("{:?}", columns);
    assert_eq!(columns.len(), 3);
    assert_eq!(columns[0], ["1", "2", "3"]);
    assert_eq!(columns[2], ["Support", "ADC", "Fighter"]);
}

fn widths_of(columns: &Vec<Vec<String>>) -> Vec<usize> {
    let mut v = Vec::new();
    for column in columns {
        v.push(column.iter().map(|e| e.len()).max().unwrap())
    }
    println! {"{:?}", v};

    v
}

fn print_one_line(columns: &Vec<String>, widths: &Vec<usize>) -> () {
    let mut v = Vec::new();
    for (column, width) in columns.iter().zip(widths) {
        let mut spaces = String::new();
        if width > column.len().borrow() {
            for _ in 0..(width - column.len()) {
                spaces.push(' ')
            }
        }
        v.push(format!("{}{} |", column, spaces));
    }
    println!("{}\n", v.join(""));
}

fn print_columns(columns: Vec<Vec<String>>, width: &Vec<usize>) {
    for i in 0..columns.first().unwrap().len() {
        print_one_line(
            &columns.iter().map(|column| column[i].clone()).collect(),
            width,
        )
    }
}
