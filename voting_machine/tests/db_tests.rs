use voting_machine::db::{initialize_db, register_voter, is_voter_registered, cast_vote, record_vote, has_voted};
use rusqlite::{Connection, params};

// Helper function to create an in-memory database for testing
fn create_test_connection() -> Connection {
    let conn = initialize_db().unwrap();
    conn
}

#[test]
fn test_initialize_db() {
    let conn = create_test_connection();
    assert!(conn.is_autocommit(), "Database should initialize correctly with autocommit mode.");
}

#[test]
fn test_register_voter() {
    let conn = create_test_connection();
    let name = "John Doe";
    let dob = "01/01/1990";

    register_voter(&conn, name, dob).unwrap();
    let exists = is_voter_registered(&conn, name, dob).unwrap();
    assert!(exists, "Voter should be registered successfully.");
}

#[test]
fn test_register_duplicate_voter() {
    let conn = create_test_connection();
    let name = "Jane Doe";
    let dob = "02/02/1992";

    register_voter(&conn, name, dob).unwrap();
    let result = register_voter(&conn, name, dob);
    assert!(result.is_ok(), "Duplicate registration should not cause an error.");
}

#[test]
fn test_has_voted() {
    let conn = create_test_connection();
    let voter_name = "John Doe";
    let voter_dob = "01/01/1990";
    let office_name = "President";

    register_voter(&conn, voter_name, voter_dob).unwrap();
    conn.execute("INSERT INTO offices (name) VALUES (?1)", params![office_name]).unwrap();

    let voter_id = conn
        .query_row("SELECT id FROM voters WHERE name = ?1", params![voter_name], |row| row.get(0))
        .unwrap();
    let office_id = conn
        .query_row("SELECT id FROM offices WHERE name = ?1", params![office_name], |row| row.get(0))
        .unwrap();

    record_vote(&conn, voter_id, office_id).unwrap();

    // Set promo_eligible to false (default case)
    let already_voted = has_voted(&conn, voter_id, office_id, false).unwrap();
    assert!(already_voted, "Voter should be marked as having voted.");
}
