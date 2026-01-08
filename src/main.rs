use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::{self};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    name: String,
    fields: HashMap<String, String>, // Schema: "age" -> "int"
    columns: Vec<String>,            // KEEPS ORDER: ["id", "name", "age"]
    data: HashMap<String, Vec<DataType>>, 
}


fn tokenize(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}


fn create_table(name: &str, cols: Vec<(&str, &str)>) {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut data: HashMap<String, Vec<DataType>> = HashMap::new();
    let mut columns: Vec<String> = Vec::new(); // Store order

    for (col, data_type) in cols {
        fields.insert(col.to_string(), data_type.to_string());
        columns.push(col.to_string());
        data.insert(col.to_string(), Vec::new());
    }

    let table = Table {
        name: name.to_string(),
        fields,
        columns,
        data,
    };

    save_table(&table);
    println!("Table '{}' created", name);
}


fn drop_table(name: &str) {
    let path = format!("data/{}.json", name);
    if std::fs::remove_file(path).is_ok() {
        println!("Table '{}' dropped", name);
    }
    else {
        println!("Table '{}' does not exists!", name);
    }
}

fn show_tables() {
    if let Ok(entries) = fs::read_dir("data") {
        for e in entries {
            let path = e.unwrap().path();
            if path.extension().unwrap_or_default() == "json" {
                println!("{}", path.file_stem().unwrap().to_str().unwrap());
            }
        }
    }
}


fn insert_row(table_name: &str, values: Vec<&str>) {
    let mut table = load_table(table_name);

    // Check if input count matches column count
    if values.len() != table.columns.len() {
        println!("Error: Column count mismatch.");
        return;
    }

    // Iterate the columns
    for (i, col_name) in table.columns.iter().enumerate() {
        let target_type = table.fields.get(col_name).unwrap();
        let val = parse_value(target_type, values[i]);
        
        table.data.get_mut(col_name).unwrap().push(val);
    }

    save_table(&table);
    println!("1 row inserted");
}

fn select_all(table_name: &str) {
    let table = load_table(table_name);
    
    // Print Header
    for col in &table.columns {
        print!("{:15}", col);
    }
    println!();
    println!("{}", "-".repeat(table.columns.len() * 15));

    // Get row count from the first column
    let row_count = if let Some(first_col) = table.columns.first() {
        table.data.get(first_col).unwrap().len()
    } else { 
        0 
    };

    // Print Rows
    for i in 0..row_count {
        for col in &table.columns {
            // Simplified print for demo
            match &table.data[col][i] {
                DataType::Integer32(v) => print!("{:15} ", v),
                DataType::Float32(v) => print!("{:15} ", v),
                DataType::String(v) => print!("{:15} ", v),
            }
        }
        println!();
    }
}


fn select_where(table_name: &str, col_name: &str, target_id: i32) {
    let table = load_table(table_name);
    
    // Get the column to search
    if let Some(column_data) = table.data.get(col_name) {

        // Find the index where the data matches our target
        let mut found_index = None;
        for (i, data) in column_data.iter().enumerate() {
            if let DataType::Integer32(val) = data {
                if *val == target_id {
                    found_index = Some(i);
                    break;
                }
            }
        }

        // If found, print that index for ALL columns
        match found_index {
            Some(i) => {
                for col in &table.columns {
                    print!("{:?} ", table.data[col][i]);
                }
                println!();
            },
            None => println!("No row found with {} = {}", col_name, target_id),
        }
    } else {
        println!("Column {} not found", col_name);
    }
}

fn print_help() {
    println!("DDL:");
    println!("  CREATE TABLE <name>");
    println!("  DROP TABLE <name>");
    println!("  SHOW TABLES\n");

    println!("DML:");
    println!("  INSERT INTO <table> VALUES <id> <name>");
    println!("  SELECT * FROM <table>");
    println!("  SELECT * FROM <table> WHERE id = <id>");
}

fn save_table(table: &Table) {
    let file = std::fs::File::create(format!("data/{}.json", table.name)).unwrap();
    serde_json::to_writer_pretty(file, table).unwrap();
}

fn load_table(name: &str) -> Table {
    let file = std::fs::File::open(format!("data/{}.json", name)).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn parse_value(typ: &str, raw: &str) -> DataType {
    match typ {
        "int" => DataType::Integer32(raw.parse().unwrap()),
        "float" => DataType::Float32(raw.parse().unwrap()),
        _ => DataType::String(raw.to_string()),
    }
}


fn main() {
    loop {
        print!("dbms> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let t: Vec<&str> = tokenize(&input);
        
    
        match t.as_slice() {
            ["CREATE", "TABLE", table, rest @ ..] => {
                let mut cols = Vec::new();
                let mut syntax_error = false;

                for c in rest {
                    let parts: Vec<&str> = c.split(':').collect();
                    
                    // Ensure we have exactly [name, type]
                    if parts.len() == 2 {
                        cols.push((parts[0], parts[1]));
                    } else {
                        println!("Syntax Error: Column '{}' format is invalid. Use name:type", c);
                        syntax_error = true;
                        break;
                    }
                }

                // Only create the table if there were no errors
                if !syntax_error {
                    create_table(table, cols);
                }
            }

            // SHOW TABLES
            ["SHOW", "TABLES"] => show_tables(),
            ["DROP", "TABLE", table] => drop_table(table),

            ["INSERT", table, values @ ..] => {
                insert_row(table, values.to_vec());
            }
            ["SELECT", "*", "FROM", table] => {
                select_all(table);
            }

            // SELECT * FROM users WHERE id = 1
            ["SELECT", "*", "FROM", table, "WHERE", col, "=", val] => {
                if let Ok(id) = val.parse::<i32>() {
                    select_where(table, col, id);
                } else {
                    println!("Only integer search supported currently.");
                }
            }


            ["HELP"] => print_help(),
            ["EXIT"] => break,

            _ => println!("Invalid command"),
        }
    }
}
