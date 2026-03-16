use voting_machine::{Ballot, open_ballot, close_ballot, verify_voter};
use voting_machine::db::{initialize_db, register_voter};
use rusqlite::Connection;

#[test]
fn test_open_ballot() {
    // Scenario: Ensure calling `open_ballot` opens the ballot if it's initially closed.
    let mut ballot = Ballot {
        offices: vec![],
        is_open: false,
    };
    open_ballot(&mut ballot);
    assert!(ballot.is_open, "The ballot should be open after calling `open_ballot`.");
}

#[test]
fn test_open_ballot_already_open() {
    // Scenario: Ensure calling `open_ballot` does not alter the ballot if it's already open.
    let mut ballot = Ballot {
        offices: vec![],
        is_open: true,
    };
    open_ballot(&mut ballot);
    assert!(ballot.is_open, "The ballot should remain open if it's already open.");
}

#[test]
fn test_close_ballot() {
    // Scenario: Ensure calling `close_ballot` closes the ballot if it's initially open.
    let mut ballot = Ballot {
        offices: vec![],
        is_open: true,
    };
    close_ballot(&mut ballot);
    assert!(!ballot.is_open, "The ballot should be closed after calling `close_ballot`.");
}

#[test]
fn test_close_ballot_already_closed() {
    // Scenario: Ensure calling `close_ballot` does not alter the ballot if it's already closed.
    let mut ballot = Ballot {
        offices: vec![],
        is_open: false,
    };
    close_ballot(&mut ballot);
    assert!(!ballot.is_open, "The ballot should remain closed if it's already closed.");
}

#[test]
fn test_verify_voter_registered() {
    // Scenario: Ensure `verify_voter` correctly identifies a registered voter.
    let conn = initialize_db().expect("Failed to initialize database.");
    let voter_name = "Jane Doe";
    let voter_dob = "02/02/1990";

    // Register a voter in the database
    register_voter(&conn, voter_name, voter_dob).expect("Failed to register voter.");

    // Verify the voter
    let is_verified = verify_voter(&conn, voter_name, voter_dob);
    assert!(is_verified, "Registered voter should be successfully verified.");
}

#[test]
fn test_verify_voter_unregistered() {
    // Scenario: Ensure `verify_voter` correctly identifies an unregistered voter.
    let conn = initialize_db().expect("Failed to initialize database.");
    let voter_name = "Unregistered Voter";
    let voter_dob = "01/01/2000";

    // Verify the voter
    let is_verified = verify_voter(&conn, voter_name, voter_dob);
    assert!(!is_verified, "Unregistered voter should not be verified.");
}

#[test]
fn test_verify_voter_invalid_date() {
    // Scenario: Ensure `verify_voter` handles invalid date formats gracefully.
    let conn = initialize_db().expect("Failed to initialize database.");
    let voter_name = "Invalid Date User";
    let invalid_dob = "31-12-2000"; // Incorrect format

    // Verify the voter
    let is_verified = verify_voter(&conn, voter_name, invalid_dob);
    assert!(
        !is_verified,
        "Voter verification should fail for incorrectly formatted dates."
    );
}
