use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Customer {
    customer_id: i32,
    first_name: String,
    last_name: String,
    gender: String,
    household_income: f64,
    birthdate: String,
    phone_number: String,
    email: String,
}

fn main() -> Result<()> {
    // Open or create the SQLite database.
    let conn = Connection::open("Car_Database.db")?;

    // Create a 'Customers' table if it doesn't exist.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Customers (
            customer_id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            gender TEXT NOT NULL,
            household_income REAL NOT NULL,
            birthdate TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;

    // Insert data into the 'Customers' table (Create).
    let customer = Customer {
        customer_id: 9,
        first_name: "Francisco".to_string(),
        last_name: "Martinez".to_string(),
        gender: "Male".to_string(),
        household_income: 75000.0,
        birthdate: "1990-05-15".to_string(),
        phone_number: "5561234567".to_string(),
        email: "francisco@example.com".to_string(),
    };

    conn.execute(
        "INSERT INTO Customers (first_name, last_name, gender, household_income, birthdate, phone_number, email) VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            customer.first_name,
            customer.last_name,
            customer.gender,
            customer.household_income,
            customer.birthdate,
            customer.phone_number,
            customer.email
        ],
    )?;

    // Define the second customer
    let customer2 = Customer {
        customer_id: 11, // Use a unique ID
        first_name: "Juan".to_string(),
        last_name: "Valdez".to_string(),
        gender: "Male".to_string(),
        household_income: 75000.0,
        birthdate: "1995-06-20".to_string(),
        phone_number: "4311234567".to_string(),
        email: "juan@example.com".to_string(),
    };
    
    // Insert the second customer into the 'Customers' table
    conn.execute(
        "INSERT INTO Customers (first_name, last_name, gender, household_income, birthdate, phone_number, email) VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            customer2.first_name,
            customer2.last_name,
            customer2.gender,
            customer2.household_income,
            customer2.birthdate,
            customer2.phone_number,
            customer2.email
        ],
    )?;
    

    // Read data from the 'Customers' table (Read).
    let mut stmt = conn.prepare("SELECT customer_id, first_name, last_name, gender, household_income, birthdate, phone_number, email FROM Customers")?;
    let customer_iter = stmt.query_map([], |row| {
        Ok(Customer {
            customer_id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            gender: row.get(3)?,
            household_income: row.get(4)?,
            birthdate: row.get(5)?,
            phone_number: row.get(6)?,
            email: row.get(7)?,
        })
    })?;

    println!("Customers in the database:");
    for customer in customer_iter {
        println!("{:?}", customer?);
    }

    // Update data in the 'Customers' table (Update).
    conn.execute(
        "UPDATE Customers SET household_income = ? WHERE first_name = ? AND last_name = ?",
        params![85000.0, "John", "Doe"],
    )?;

    // Read updated data.
    let updated_customer = conn.query_row(
        "SELECT customer_id, first_name, last_name, gender, household_income, birthdate, phone_number, email FROM Customers WHERE first_name = ? AND last_name = ?",
        params!["John", "Doe"],
        |row| {
            Ok(Customer {
                customer_id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                gender: row.get(3)?,
                household_income: row.get(4)?,
                birthdate: row.get(5)?,
                phone_number: row.get(6)?,
                email: row.get(7)?,
            })
        },
    )?;
    println!("Updated Customer: {:?}", updated_customer);

    // Delete data from the 'Customers' table (Delete).
    conn.execute(
        "DELETE FROM Customers WHERE first_name = ? AND last_name = ?",
        params!["Shaila", "Doe"],
    )?;

    // Verify deletion.
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM Customers", [], |row| row.get(0))?;
    println!("Number of Customers after Deletion: {}", count);

    Ok(())
}
