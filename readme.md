# RustDBMS

A lightweight, **columnar database management system** built from scratch in Rust.
This project was designed to explore database internals, specifically **column-oriented storage**, persistent serialization, and building a custom SQL-like command interpreter.

## Features

- **Columnar Storage Engine:** Stores data by columns (vectors) rather than rows for efficient aggregation.
- **Persistent Storage:** Saves tables as JSON files using `serde`.
- **SQL-Like Interface:** Supports DDL and DML commands.
- **Formatted Output:** Uses `prettytable-rs` for CLI visualization.
- **Dockerized:** Ready for containerized deployment.
- **Type System:** Supports `Integer32`, `Float32`, and `String` with strong type validation.

---

## Installation & Usage

### Option 1: Running with Rust (Recommended for Dev)

Ensure you have [Rust](https://www.rust-lang.org/) installed.

```bash
# Clone the repository
git clone <your-repo-url>
cd rust-dbms

# Run the engine
cargo run

```

### Option 2: Running with Docker (Recommended for Deploy)

No Rust installation required.

```bash
# Build the image
docker build -t rust-dbms .

# Run with persistence (Windows PowerShell)
docker run -it -v ${PWD}/data:/app/data rust-dbms

# Run with persistence (Linux/Mac)
docker run -it -v $(pwd)/data:/app/data rust-dbms

```

---

## Command Reference

### Data Definition (DDL)

| Command          | Description                             | Example                                         |
| ---------------- | --------------------------------------- | ----------------------------------------------- |
| **CREATE TABLE** | Creates a new table with typed columns. | `CREATE TABLE users id:int name:string age:int` |
| **DROP TABLE**   | Deletes a table and its data file.      | `DROP TABLE users`                              |
| **SHOW TABLES**  | Lists all existing tables.              | `SHOW TABLES`                                   |

### Data Manipulation (DML)

| Command          | Description                                  | Example                            |
| ---------------- | -------------------------------------------- | ---------------------------------- |
| **INSERT**       | Adds a row. (Must match column order/types). | `INSERT users 1 harsh 25`          |
| **SELECT**       | Prints all rows in the table.                | `SELECT * FROM users`              |
| **SELECT WHERE** | Finds rows by integer value (Indexed Scan).  | `SELECT * FROM users WHERE id = 1` |
| **DELETE**       | Removes a row by ID.                         | `DELETE FROM users WHERE id = 1`   |
| **COUNT**        | Returns the total number of rows.            | `COUNT users`                      |

---

## Architecture

### 1. Storage Format (Columnar)

Unlike traditional row-stores (e.g., PostgreSQL), RustDBMS stores data in columns. This makes aggregations (like `COUNT` or `SUM`) extremely fast as the engine only reads the specific vector needed.

**Internal Structure:**

```rust
enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

pub struct Table {
    name: String,                         // Table name
    fields: HashMap<String, String>,      // Schema: Field name : DataType (ex: "age" : "int")
    columns: Vec<String>,                 // KEEPS ORDER: ["id", "name", "age"]
    data: HashMap<String, Vec<DataType>>, // Column name -> {Vector containing data in order of row}
}

```

### 2. Persistence

Data is serialized to `.json` files in the `data/` directory.

- **Read:** Loads the entire JSON into memory on `load_table`.
- **Write:** Serializes the struct back to JSON on every `INSERT`/`DELETE`.

---

## Demo

Here is the DBMS running in the terminal:

![RustDBMS CLI Screenshot](assets/demo.png)

emp.json generated for persistence

```json
{
  "name": "emp",
  "fields": {
    "id": "int",
    "salary": "float",
    "name": "string",
    "age": "int"
  },
  "columns": ["id", "name", "age", "salary"],
  "data": {
    "age": [
      {
        "Integer32": 24
      },
      {
        "Integer32": 28
      }
    ],
    "name": [
      {
        "String": "Max"
      },
      {
        "String": "Daniel"
      }
    ],
    "id": [
      {
        "Integer32": 2
      },
      {
        "Integer32": 3
      }
    ],
    "salary": [
      {
        "Float32": 12.0
      },
      {
        "Float32": 22.5
      }
    ]
  }
}
```

## Future Roadmap

- Implement **B-Tree Indexing** for faster lookups (avoid full scans).
- Support string/float in `WHERE` clauses.

---
