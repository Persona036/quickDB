use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: BTreeMap<String, Vec<DataType>>,
}
pub fn handle_command(command: &str, tables: &mut BTreeMap<String, Table>) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return "Please specify a command".to_string();
    }

    match parts[0] {
        "create" => {
            if parts.len() < 2 {
                return "Usage: create <table_name>".to_string();
            }

            let table_name = parts[1].to_string();
            tables.insert(
                table_name.clone(),
                Table {
                    name: table_name,
                    columns: BTreeMap::new(),
                },
            );
            format!("Table {} created", parts[1])
        }
        "insert" => {
            if parts.len() < 3 {
                return "Usage: insert <table_name> <column_name> <value>".to_string();
            }
            let table_name = parts[1].to_string();
            let column_name = parts[2].to_string();
            let value = parts[3..].join("");

            if let Some(table) = tables.get_mut(&table_name) {
                let data_type = if let Ok(int_val) = value.parse::<i32>() {
                    DataType::Integer32(int_val)
                } else if let Ok(float_val) = value.parse::<f32>() {
                    DataType::Float32(float_val)
                } else {
                    DataType::String(value)
                };
                let column = table
                    .columns
                    .entry(column_name.to_string())
                    .or_insert(Vec::new());
                column.push(data_type);

                format!("Inserted into {} {}", table_name, column_name)
            } else {
                format!("Table {} not found", table_name)
            }
        }
        "select" => {
            if parts.len() < 2 {
                return "Usage: select <table_name> <column>".to_string();
            }
            let table_name = parts[1].to_string();

            if let Some(table) = tables.get(&table_name) {
                if parts.len() == 3 {
                    let column_name = parts[2].to_string();
                    if let Some(col_data) = table.columns.get(&column_name) {
                        format!("{}: {:?}\n", column_name, col_data)
                    } else {
                        format!("Column {} not found in table {}", column_name, table_name)
                    }
                } else {
                    let mut result = String::new();
                    for (col_name, col_data) in &table.columns {
                        result.push_str(&format!("{}: {:?}\n", col_name, col_data));
                    }
                    result
                }
            } else {
                format!("Table {} not found", table_name)
            }
        }
        "save" => {
            if parts.len() < 2 {
                return "Usage: save <file_name>".to_string();
            }
            let file_name = parts[1];
            match save_to_file(tables, file_name) {
                Ok(_) => format!("Database saved to {}", file_name),
                Err(e) => format!("Failed to save Database: {}", e),
            }
        }
        "load" => {
            if parts.len() < 2 {
                return "Usage: load <file_name>".to_string();
            }
            let file_name = parts[1];

            match load_from_file(file_name) {
                Ok(loaded_tables) => {
                    *tables = loaded_tables;
                    format!("Successfully loaded database from {}", parts[1])
                }
                Err(e) => format!("Failed to load Database: {}", e),
            }
        }
        _ => "Unknown Command".to_string(),
    }
}

pub fn save_to_file(
    tables: &BTreeMap<String, Table>,
    file_name: &str,
) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(tables)?;
    std::fs::write(file_name, serialized)?;
    Ok(())
}

pub fn load_from_file(file_name: &str) -> Result<BTreeMap<String, Table>, std::io::Error> {
    let data = std::fs::read_to_string(file_name)?;
    let tables: BTreeMap<String, Table> = serde_json::from_str(&data)?;

    Ok(tables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let mut tables = BTreeMap::new();
        let command = "create test_table";
        let result = handle_command(command, &mut tables);
        assert_eq!(result, "Table test_table created");
        assert!(tables.contains_key("test_table"));
    }

    #[test]
    fn test_insert_and_select() {
        let mut tables = BTreeMap::new();
        handle_command("create test_table", &mut tables);
        handle_command("insert test_table name Alice", &mut tables);
        handle_command("insert test_table age 30", &mut tables);

        let select_name = handle_command("select test_table name", &mut tables);
        assert_eq!(select_name, "name: [String(\"Alice\")]\n");

        let select_age = handle_command("select test_table age", &mut tables);
        assert_eq!(select_age, "age: [Integer32(30)]\n");
    }

    #[test]
    fn test_insert_multiple_values() {
        let mut tables = BTreeMap::new();
        handle_command("create test_table", &mut tables);
        handle_command("insert test_table name Alice", &mut tables);
        handle_command("insert test_table name Bob", &mut tables);

        let select_name = handle_command("select test_table name", &mut tables);
        assert_eq!(select_name, "name: [String(\"Alice\"), String(\"Bob\")]\n");
    }

    #[test]
    fn test_select_non_existent_column() {
        let mut tables = BTreeMap::new();
        handle_command("create test_table", &mut tables);
        let result = handle_command("select test_table non_existent", &mut tables);
        assert_eq!(result, "Column non_existent not found in table test_table");
    }

    #[test]
    fn test_save_and_load() {
        let mut tables = BTreeMap::new();
        handle_command("create test_table", &mut tables);
        handle_command("insert test_table name Alice", &mut tables);
        handle_command("save test_db.json", &mut tables);

        let mut new_tables = BTreeMap::new();
        handle_command("load test_db.json", &mut new_tables);
        let select_name = handle_command("select test_table name", &mut new_tables);
        assert_eq!(select_name, "name: [String(\"Alice\")]\n");
    }
}
