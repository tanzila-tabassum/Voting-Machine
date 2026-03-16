pub mod db; // This line is to make the db.rs file accessible


use rusqlite::Connection;

#[derive(Debug)]
pub struct Ballot {
    pub offices: Vec<Office>,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Office {
    pub name: String,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug)]
pub struct Candidate {
    pub name: String,
    pub party: String,
    pub votes: u32,
}

// Make functions public for tests
pub fn open_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        println!("The election is already open.");
    } else {
        ballot.is_open = true;
        println!("The election has been opened for voting.");
    }
}

pub fn close_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        ballot.is_open = false;
        println!("The ballot has been closed. No voting possible!");
    } else {
        println!("The election is already closed.");
    }
}

// Verifies if a voter is registered
pub fn verify_voter(conn: &Connection, voter_name: &str, voter_dob: &str) -> bool {
    match db::is_voter_registered(conn, voter_name, voter_dob) {
        Ok(true) => {
            println!("Welcome, {}! You are verified to vote.", voter_name);
            true
        }
        Ok(false) => {
            println!("You are not registered for voting.");
            false
        }
        Err(err) => {
            println!("Error verifying voter: {}", err);
            false
        }
    }
}
