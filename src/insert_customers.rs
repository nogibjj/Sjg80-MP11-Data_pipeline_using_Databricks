use rusqlite::{params, Connection, Result, Error};

fn main() -> Result<()> {
    // Open or create the SQLite database.
    let conn = Connection::open("Car_Database.db")?;

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

    let customer1 = Customer {
        customer_id: 10,
        first_name: "Michael".to_string(),
        last_name: "Jackson".to_string(),
        gender: "Female".to_string(),
        household_income: 60000.0,
        birthdate: "2020-12-03".to_string(),
        phone_number: "5555476543".to_string(),
        email: "michael@example.com".to_string(),
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
        customer_id: 11,
        first_name: "Bob".to_string(),
        last_name: "Sponge".to_string(),
        gender: "Male".to_string(),
        household_income: 75000.0,
        birthdate: "1940-06-20".to_string(),
        phone_number: "5561204567".to_string(),
        email: "bob@example.com".to_string(),
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
