use serde_json::Value;
use std::borrow::Borrow;

pub fn print_as_table(rows: Value, headers: Vec<&str>) -> () {
    let columns = split_columns(rows, headers);
    let widths = widths_of(&columns);

    print_columns(columns, &widths);
}

fn split_columns(rows: Value, headers: Vec<&str>) -> Vec<Vec<String>> {
    let mut vec = Vec::new();
    for header in headers {
        let mut column = Vec::new();
        column.push(header.to_string());
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
    assert_eq!(columns.len(), 3);
    assert_eq!(columns[0], ["number", "1", "2", "3"]);
    assert_eq!(columns[2], ["role", "Support", "ADC", "Fighter"]);
}

fn widths_of(columns: &Vec<Vec<String>>) -> Vec<usize> {
    let mut v = Vec::new();
    for column in columns {
        v.push(column.iter().map(|e| e.len()).max().unwrap())
    }
    v
}

#[test]
fn widths_of_test() {
    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![
            "Thresh".to_string(),
            "Lucian".to_string(),
            "Shyvana".to_string(),
        ],
        vec![
            "Support".to_string(),
            "ADC".to_string(),
            "Fighter".to_string(),
        ],
    ];

    let widths = widths_of(&data);
    assert_eq!(widths, vec![1, 7, 7]);
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
