use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Car {
    id: i32,
    make: String,
    model: String,
    year: i32,
}

fn main() -> Result<()> {
    // Open or create the SQLite database.
    let conn = Connection::open("basededatos.db")?;

    // Create a 'cars' table if it doesn't exist.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cars (
            id INTEGER PRIMARY KEY,
            make TEXT NOT NULL,
            model TEXT NOT NULL,
            year INTEGER NOT NULL
        )",
        [],
    )?;

    // Insert data into the 'cars' table (Create).
    let car = Car {
        id: 1,
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    };

    conn.execute(
        "INSERT INTO cars (make, model, year) VALUES (?, ?, ?)",
        params![car.make, car.model, car.year],
    )?;

    // Read data from the 'cars' table (Read).
    let mut stmt = conn.prepare("SELECT id, make, model, year FROM cars")?;
    let car_iter = stmt.query_map([], |row| {
        Ok(Car {
            id: row.get(0)?,
            make: row.get(1)?,
            model: row.get(2)?,
            year: row.get(3)?,
        })
    })?;

    println!("Cars in the database:");
    for car in car_iter {
        println!("{:?}", car?);
    }

    // Update data in the 'cars' table (Update).
    conn.execute(
        "UPDATE cars SET year = ? WHERE make = ? AND model = ?",
        params![2019, "Toyota", "Camry"],
    )?;

    // Read updated data.
    let updated_car = conn.query_row(
        "SELECT id, make, model, year FROM cars WHERE make = ? AND model = ?",
        params!["Toyota", "Camry"],
        |row| {
            Ok(Car {
                id: row.get(0)?,
                make: row.get(1)?,
                model: row.get(2)?,
                year: row.get(3)?,
            })
        },
    )?;
    println!("Updated Car: {:?}", updated_car);

    // Delete data from the 'cars' table (Delete).
    conn.execute(
        "DELETE FROM cars WHERE make = ? AND model = ?",
        params!["Toyota", "Camry"],
    )?;

    // Verify deletion.
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM cars", [], |row| row.get(0))?;
    println!("Number of Cars after Deletion: {}", count);

    Ok(())
}
