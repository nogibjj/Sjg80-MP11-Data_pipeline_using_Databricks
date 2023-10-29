use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Open or create the SQLite database.
    let conn = Connection::open("Car_Database.db")?;

    #[derive(Debug)]
    struct Customer {
        first_name: String,
        last_name: String,
        gender: String,
        household_income: f64,
        birthdate: String,
        phone_number: String,
        email: String,
    }

    let customer1 = Customer {
        first_name: "Katy".to_string(),
        last_name: "Perry".to_string(),
        gender: "Female".to_string(),
        household_income: 60000.0,
        birthdate: "1994-12-03".to_string(),
        phone_number: "2345476543".to_string(),
        email: "britney@example.com".to_string(),
    };

    // Insert the first customer into the 'Customers' table
    conn.execute(
        "INSERT INTO Customers (first_name, last_name, gender, household_income, birthdate, phone_number, email) VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            customer1.first_name,
            customer1.last_name,
            customer1.gender,
            customer1.household_income,
            customer1.birthdate,
            customer1.phone_number,
            customer1.email
        ],
    )?;

    let customer2 = Customer {
        first_name: "Bruno".to_string(),
        last_name: "Mars".to_string(),
        gender: "Male".to_string(),
        household_income: 75000.0,
        birthdate: "1940-01-29".to_string(),
        phone_number: "1231204567".to_string(),
        email: "paul@example.com".to_string(),
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

    Ok(())
}
