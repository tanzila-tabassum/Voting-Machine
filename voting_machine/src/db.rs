

// ================= This file manages the database, the creation of tables, the insertion and deletion of the data ======================

use chrono::{Datelike, NaiveDate}; 
use rusqlite::{params, Connection, Result};

// Voter struct for database interaction
pub struct Voter {
    pub id: i32,
    pub name: String,
    pub date_of_birth: String,
    pub has_voted: bool,
}

// Initialize the database and create tables if they don't exist
pub fn initialize_db() -> Result<Connection> {
    // Open a connection to the SQLite database file
    let conn = Connection::open("voting_machine.db")?;

    println!("Database initialized successfully!");

    // Create voters table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS voters (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL,
             date_of_birth TEXT NOT NULL,
             has_voted BOOLEAN NOT NULL DEFAULT 0,
             UNIQUE(name, date_of_birth)
         )",
        [],
    )?;

    // Create offices table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS offices (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    // Create candidates table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidates (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             party TEXT NOT NULL,
             votes INTEGER NOT NULL DEFAULT 0,
             office_id INTEGER,
             FOREIGN KEY(office_id) REFERENCES offices(id)
         )",
        [],
    )?;

    // Create votes table to ensure each voter can vote only once per office
conn.execute(
    "CREATE TABLE IF NOT EXISTS votes (
         id INTEGER PRIMARY KEY,
         voter_id INTEGER,
         office_id INTEGER,
         FOREIGN KEY(voter_id) REFERENCES voters(id),
         FOREIGN KEY(office_id) REFERENCES offices(id)
     )",
    [],
)?;

    Ok(conn) // Return the connection after setting up tables
}

// Register a new voter in the database
pub fn register_voter(conn: &Connection, name: &str, date_of_birth: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM voters WHERE name = ?1 COLLATE NOCASE AND date_of_birth = ?2")?;
    let voter_exists: i32 = stmt.query_row(params![name, date_of_birth], |row| row.get(0))?;

    if voter_exists > 0 {
        println!("\n\tVoter '{}' with the date of birth '{}' is already registered!", name, date_of_birth);
        return Ok(()); 
    }

    conn.execute(
        "INSERT INTO voters (name, date_of_birth, has_voted) VALUES (?1, ?2, ?3)",
        params![name, date_of_birth, false],
    )?;

    println!("Voter '{}' registered successfully!", name);
    Ok(())
}


// Check if a voter is registered
pub fn is_voter_registered(conn: &Connection, name: &str, dob: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM voters WHERE name = ?1 AND date_of_birth = ?2)")?;
    let exists: bool = stmt.query_row(params![name, dob], |row| row.get(0))?;
    Ok(exists)
}

// Cast a vote by incrementing the vote count for a candidate
pub fn cast_vote(conn: &Connection, candidate_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE candidates SET votes = votes + 1 WHERE name = ?1",
        params![candidate_name],
    )?;
    println!("Vote cast for candidate {}", candidate_name);
    Ok(())
}

// Cast a vote by updating the vote count for a specific candidate by id
pub fn cast_vote_by_id(conn: &Connection, candidate_id: i32) -> Result<()> {
    conn.execute(
        "UPDATE candidates SET votes = votes + 1 WHERE id = ?1",
        params![candidate_id],
    )?;
    println!("Vote cast for candidate with id {}", candidate_id);
    Ok(())
}

// Check if a voter has already voted for a specific office
pub fn has_voted(conn: &Connection, voter_id: i32, office_id: i32, promo_eligible: bool) -> Result<bool> {
    if promo_eligible {
        return Ok(false); 
    }

    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM votes WHERE voter_id = ?1 AND office_id = ?2)")?;
    let already_voted: bool = stmt.query_row(params![voter_id, office_id], |row| row.get(0))?;
    Ok(already_voted)
}

// Record a new vote in the votes table
pub fn record_vote(conn: &Connection, voter_id: i32, office_id: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO votes (voter_id, office_id) VALUES (?1, ?2)",
        params![voter_id, office_id],
    )?;
    Ok(())
}

// Retrieve the birth year of a voter by their ID
pub fn get_voter_birth_year(conn: &Connection, voter_id: i32) -> Result<i32, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT date_of_birth FROM voters WHERE id = ?1")?;
    let date_of_birth: String = stmt.query_row(params![voter_id], |row| row.get(0))?;
    
    // Parse the birth year from the date of birth string
    if let Ok(parsed_date) = NaiveDate::parse_from_str(&date_of_birth, "%m/%d/%Y") {
        Ok(parsed_date.year())
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}