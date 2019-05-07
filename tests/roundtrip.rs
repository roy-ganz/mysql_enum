
use mysql;
use mysql::params;

use mysql_enum::MysqlEnum;
use strum_macros::{Display, EnumString};

use std::env;

#[derive(Debug, PartialEq, EnumString, Display, MysqlEnum)]
pub enum UserRole {
    Admin,
    User,
}

pub struct User {
    id: u64,
    role: UserRole,
}

#[test]
fn roundtrip() {

    let k = match env::var("LOGIN") {
        Ok(val) => val,
        Err(_) => panic!("Missing var, run with `LOGIN=<username>:<password> cargo test`"),
    };

    let mut conn = mysql::Conn::new(format!("mysql://{}@localhost:3306/mysql",k)).unwrap();

    // Create temporary table
    conn.prep_exec(
        r"CREATE TEMPORARY TABLE User (
                         id int not null,
                         role enum('Admin','User') not null
                     )",
        (),
    )
    .expect("Create statement is not valid.");
    

    // Create user with enum field
    let u = User {
        id: 0,
        role: UserRole::Admin,
    };

    // Insert user into temporary table
    {
        let mut stmt = conn
            .prepare(r"INSERT INTO User  (id, role) VALUES (:id, :role)")
            .expect("Insert statement is not valid.");

        stmt.execute(params! {
            "id" => u.id,
            "role" => u.role
        })
        .unwrap();
    }

    // Query all users
    let users: Vec<User> = conn
        .prep_exec("SELECT id, role from User", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, role) = mysql::from_row(row);
                    User { id: id, role: role }
                })
                .collect()
        })
        .expect("Select not valid.");

    // Assert one user with enum
    assert_eq!(1, users.len());
    assert_eq!(UserRole::Admin, users.get(0).unwrap().role);
}
