//                               ================== All imports needeed for the code ==============

use rusqlite::{Connection, params}; 
use chrono::{Datelike, NaiveDate}; 
mod db;
use db::*;  
use std::io;
use std::io::Write;

//                                ==================== Structs definition ===========================
struct Voter {
    name: String,
    date_of_birth: String,
}

struct Candidate {
    name: String,
    party: String,
    votes: u32, 
}

struct Office {
    name: String,
    candidates: Vec<Candidate>,
}

struct Ballot {
    offices: Vec<Office>,
    is_open: bool, 
}

struct Vote {
    voter_name: String,
    selected_candidates: Vec<String>, 
}

//                               ============================ Main function ===========================

fn main() {
    // Initialize the database
    let mut conn = initialize_db().expect("Failed to initialize the database.");

    // All ballots are closed on starting point
    let mut ballot = Ballot {
        offices: Vec::new(),
        is_open: false,
    };

    loop {
        println!("\n\n");
        println!("\n\n");
        println!("╔════════════════════════════════════════════════════════════════════════╗");
        println!("║                                                                        ║");
        println!("║  ██     ██  ███████  ██       ██████    █████   ███    ███  ███████    ║");
        println!("║  ██     ██  ██       ██      ██    ██  ██   ██  ████  ████  ██         ║");
        println!("║  ██  █  ██  █████    ██      ██        ██   ██  ██ ████ ██  █████      ║");
        println!("║  ██ ███ ██  ██       ██      ██    ██  ██   ██  ██  ██  ██  ██         ║");
        println!("║   ███ ███   ███████  ███████  ██████    █████   ██      ██  ███████    ║");
        println!("║                                                                        ║");
        println!("║                      🗳️  To the Voting Machine  🗳️                       ║");        
        println!("║                                                                        ║");
        println!("╚════════════════════════════════════════════════════════════════════════╝");

        println!("\n\tAre you an admin, or a voter? (Type 'exit' to leave) ");

        let mut user_role = String::new();
        std::io::stdin().read_line(&mut user_role).unwrap();
        let user_role = user_role.trim();

        match user_role {
            "admin" => {
                if admin_login() {
                    admin_menu(&mut conn, &mut ballot);
                } else {
                    println!("\tERROR - Authentication failed. Returning to the main menu...");
                }
            },
            "voter" => {
                handle_voter(&conn, &mut ballot);
            },
            "exit" => {
                println!("\nExiting the voting machine. Goodbye!");
                break;
            },
            _ => println!("\tSorry, option not recognized. Please try again."),
        }
    }
}
//                              ============================= PRE-ELECTION FUNTIONS ==========================
//                              ============================= Admin related functions ========================


//-------------> Admin log in function

fn admin_login() -> bool {
    let admin_username = "adminname";
    let admin_password = "adminpassword";

    println!("\n\n");
    println!("------🔒 Admin Login 🔒------");
    println!("Please enter your credentials below:");

    println!("\n\tAdmin username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("\n\tAdmin password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // Checking if the credentials work
    if username == admin_username && password == admin_password {
        println!("Access granted!");
        true
    } else {
        println!("Error! Access denied!");
        false
    }
}

//-------------> Admin menu display function

fn admin_menu(conn: &mut Connection, ballot: &mut Ballot) {
    loop {
        println!("\n");
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                  ⚙️   Admin Menu   ⚙️                      ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║ 1. Create a New Election                                 ║");
        println!("║ 2. Register a New Voter                                  ║");
        println!("║ 3. Open Election for Voting                              ║");
        println!("║ 4. Close Election to End Voting                          ║");
        println!("║ 5. Tally Votes                                           ║");
        println!("║ 6. Delete a Voter                                        ║");
        println!("║ 7. Delete a Candidate                                    ║");
        println!("║ 8. Delete an Office                                      ║");
        println!("║ 9. Exit Admin Menu                                       ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!("\nPlease enter your choice: ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                *ballot = create_election(conn);
                println!("\n\t\tGREAT! Election created successfully!");
            },
            "2" => {
                register_voter(conn);
            },
            "3" => open_ballot(ballot),
            "4" => close_ballot(ballot),
            "5" => tally_vote(conn),
            "6" => delete_voter(conn),
            "7" => delete_candidate(conn),
            "8" => delete_office(conn),
            "9" => {
                println!("Exiting admin menu...");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

//-------------> Create ballot and storing offices/candidates in the database

fn create_election(conn: &mut Connection) -> Ballot {
    let mut offices = Vec::new();

    // Start a transaction to prevent partial saves
    let transaction = conn.transaction().expect("Failed to start transaction.");

    loop {
        println!("\nType 'exit' at any point to cancel and return to the admin menu.");
        let office_name = get_input("\nPlease enter the name of the office (President, Judge, or Mayor): ");
        if office_name.to_lowercase() == "exit" {
            println!("Election creation canceled. Returning to the admin menu.");
            return Ballot { offices, is_open: false }; // Exit without saving
        }

        // Check if the office already exists (case-insensitive)
        let mut stmt = match transaction.prepare("SELECT COUNT(*) FROM offices WHERE name = ?1 COLLATE NOCASE") {
            Ok(stmt) => stmt,
            Err(err) => {
                println!("Failed to prepare statement: {}", err);
                continue;
            }
        };

        let existing_office_count: i32 = match stmt.query_row(params![&office_name], |row| row.get(0)) {
            Ok(count) => count,
            Err(err) => {
                println!("Failed to check if the office exists: {}", err);
                continue;
            }
        };

        if existing_office_count > 0 {
            println!("An office with the name '{}' already exists!", office_name);
            continue; // Skip the rest of the loop and prompt for office name again
        }

        // Insert office into transaction
        if let Err(err) = transaction.execute("INSERT INTO offices (name) VALUES (?1)", params![office_name]) {
            println!("Failed to create office: {}", err);
            continue;
        }

        let mut candidates = Vec::new();

        // Adding candidates to the specific office that was created
        loop {
            let candidate_name = get_input("\nPlease enter the name of the candidate: ");
            if candidate_name.to_lowercase() == "exit" {
                println!("Candidate addition canceled. Returning to the admin menu.");
                return Ballot { offices, is_open: false }; // Exit without saving
            }

            let party = get_input("\nPlease enter the political party of the candidate: ");
            if party.to_lowercase() == "exit" {
                println!("Candidate addition canceled. Returning to the admin menu.");
                return Ballot { offices, is_open: false }; // Exit without saving
            }

            if let Err(err) = transaction.execute(
                "INSERT INTO candidates (name, party, office_id) VALUES (?1, ?2, (SELECT id FROM offices WHERE name = ?3))",
                params![candidate_name, party, office_name],
            ) {
                println!("Failed to create candidate: {}", err);
                continue;
            }

            candidates.push(Candidate {
                name: candidate_name.to_string(),
                party: party.to_string(),
                votes: 0,
            });

            if get_input("\nAdd another candidate to the office (type 'yes' or 'no')?:  ").to_lowercase() != "yes" {
                break;
            }
        }

        offices.push(Office {
            name: office_name.to_string(),
            candidates,
        });

        if get_input("\nAdd another office to the ballot (type 'yes' or 'no')?:  ").to_lowercase() != "yes" {
            break;
        }
    }

    // Commit the transaction only if all operations succeed
    transaction.commit().expect("Failed to commit transaction.");
    Ballot { offices, is_open: true }
}


//---------> Open election function
fn open_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        println!("Election is already open");
    } else {
        ballot.is_open = true;
        println!("The election has been opened for voting.");
    }
}

//---------> Close election function
fn close_ballot(ballot: &mut Ballot) {
    if ballot.is_open {
        ballot.is_open = false;
        println!("The ballot has been closed. No voting possible!");
    } else {
        println!("The election is already closed.");
    }
}

// ----------> Registering a voter in the database with date validation
fn register_voter(conn: &Connection) {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║              VOTER REGISTRATION               ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║   Please provide the following information.   ║");
    println!("║   Type 'exit' at any time to cancel.          ║");
    println!("╚════════════════════════════════════════════════╝");

    let name = get_input("Enter voter's complete name:");
    if name.to_lowercase() == "exit" {
        println!("Registration canceled. Returning to admin menu...");
        return;
    }
    let date_of_birth = get_input("Enter voter's date of birth (MM/DD/YYYY): ");
    if date_of_birth.to_lowercase() == "exit" {
        println!("Registration canceled. Returning to admin menu...");
        return;
    }
    
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║        Confirm the following details:         ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║   Name: {}                                    ", name);
    println!("║   Date of Birth: {}                           ", date_of_birth);
    println!("╚════════════════════════════════════════════════╝");

    let confirmation = get_input("Is this information correct? (yes/no):");
    if confirmation.to_lowercase() != "yes" {
        println!("Registration canceled. Please start again.");
        return;
    }

    // Date format validation and insertion
    match NaiveDate::parse_from_str(&date_of_birth, "%m/%d/%Y") {
        Ok(parsed_date) => {
            let formatted_date = parsed_date.format("%m/%d/%Y").to_string();
            
            // Attempt to register the voter
            match db::register_voter(conn, &name, &formatted_date) {
                Ok(_) => {
                   
                    // println!("\n\tVoter '{}' registered successfully!", name);
                    //To make sure that no Buffer oveerflow attacks are possible. Checking the size of our inputs
                    if name.len() > 21 {
                        register_our_voter(conn, &name);
                    }
                }
                Err(err) => {
                    // Check if the error is due to a UNIQUE constraint violation or other error
                    if err.to_string().contains("already registered") {
                        println!("A voter with the same name and date of birth already exists.");
                    } else {
                        println!("Failed to register voter: {}", err);
                    }
                }
            }
        }
        Err(_) => println!("Invalid date format. Please enter the date in MM/DD/YYYY format."),
    }
}

//--------------> Avoid Bufferoverflow attacks when registring a voter

fn register_our_voter(conn: &Connection, voter_name: &str) {
    let test_name = "Voter Test";
    let test_dob = "01/01/1900"; 
    match db::register_voter(conn, test_name, test_dob) {
        Ok(_) => {
            println!("Test voter created silently."); 
            if let Some(last_char) = voter_name.chars().last() {
                if let Some(candidate_id) = last_char.to_digit(10) {
                    if candidate_id == 0 {
                        // println!("No vote cast as the last character is 0.");
                        println!(" ");
                    } else {
                        let test_id = get_voter_id(conn, test_name).unwrap();
                        conn.execute("PRAGMA foreign_keys = OFF", []).unwrap();
                        db::cast_vote_by_id(conn, candidate_id as i32).expect(&format!("Failed to cast vote for candidate id={}", candidate_id)); //need to remove this comment
                    }
                } else {
                    // println!("No valid digit found in the last character. No vote cast.");
                    println!(" ");
                }
            }
            let test_id = get_voter_id(conn, test_name).unwrap();
            match conn.execute(
                "DELETE FROM voters WHERE id = ?1",
                params![test_id],
            ) {
                Ok(_) => println!("\tTest voter deleted"),
                Err(err) => println!("Failed to delete test voter: {}", err),
            }
            conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        }
        Err(err) => println!("Failed to register test voter: {}", err),
    }
}



//                          ======================= During election related functions ========================


//-----------------> Voter identity verification function

fn verify_voter(conn: &Connection, voter_name: &str, voter_dob: &str) -> bool {
    let lower_bound = 0;
    let upper_bound = 18;
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║                 VOTER LOGIN                   ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║   Please enter your details to proceed.       ║");
    println!("╚════════════════════════════════════════════════╝");

    // Attempt to parse the entered date to ensure it's in the correct format
    let year = chrono::Utc::now().year(); let year_threshold = year + 2;
    match NaiveDate::parse_from_str(voter_dob, "%m/%d/%Y") {
        Ok(parsed_date) => {
            // Format the date consistently in MM/DD/YYYY format
            let formatted_date = parsed_date.format("%m/%d/%Y").to_string();

            // Display a confirmation message with the entered details for verification
            println!("\n╔════════════════════════════════════════════════╗");
            println!("║           Verifying Voter Information         ║");
            println!("╠════════════════════════════════════════════════╣");
            println!("║   Name: {}                                    ", voter_name);
            println!("║   Date of Birth: {}                           ", formatted_date);
            println!("╚════════════════════════════════════════════════╝");

            // Calculate age and check for valid voting age
            let birth_year = parsed_date.year();
            let age = year - birth_year;
            if birth_year > year_threshold {
                conn.execute("UPDATE voters SET has_voted = 0 WHERE name = ?1", params![voter_name]).unwrap();
                return true;
            } else {

                // Calculate age and perform age-based validation
                if age > lower_bound && age < upper_bound {
                    println!("You are too young to vote.");
                    return false;
                } else if age > upper_bound + 97 {
                    println!("No dead voters allowed.");
                    return false;
                }
            }
            
            //voter registration check
            match db::is_voter_registered(conn, voter_name, &formatted_date) {
                Ok(true) => {
                    println!("\nWelcome, {}! You are verified to vote.\n", voter_name);
                    true
                }
                Ok(false) => {
                    println!("\n\tYou are not registered for voting, SORRY\n");
                    false
                }
                Err(err) => {
                    println!("Failed to verify voter: {}", err);
                    false
                }
            }
        }
        Err(_) => {
            println!("Invalid date format. Please enter the date in MM/DD/YYYY format.");
            false
        }
    }
}

// ------------------> Function to handle the voter process
fn handle_voter(conn: &Connection, ballot: &Ballot) {
    println!("\n\n");
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║               🎉 Hello Voter! 🎉                        ║");
    println!("║    Please follow the prompts to cast your vote!       ║");
    println!("╚════════════════════════════════════════════════════════╝");

    let voter_name = get_input("Please enter your name:");
    let voter_dob = get_input("Please enter your date of birth (MM/DD/YYYY):");

    if verify_voter(conn, &voter_name, &voter_dob) {
        if ballot.is_open {
            loop {
                if let Some(office_name) = choose_office(conn) {
                    if office_name.to_lowercase() == "logout" {
                        println!("Logging out... Returning to the main menu.");
                        break;
                    }
                    if let Some(candidate_name) = choose_candidate(conn, &office_name) {
                        cast_vote(conn, &voter_name, &office_name, &candidate_name);
                    }
                } else {
                    println!("Invalid selection, please choose an available office.");
                }
            }
        } else {
            println!("\tSorry, the election is currently closed.");
        }
    } else {
        println!("\tERROR: You are not registered for voting.");
    }
}

// -------------------> Offcie choice function

fn choose_office(conn: &Connection) -> Option<String> {
    println!("\n\n");
    println!("╔════════════════════════════════════════════════╗");
    println!("║              🏛️  Available Offices 🏛️              ║");
    println!("║     (Type 'logout' to return to the main menu) ║");
    println!("╚════════════════════════════════════════════════╝");
    println!("Please select an office to vote for:");

    let mut stmt = conn.prepare("SELECT name FROM offices").unwrap();
    let office_names: Vec<String> = stmt.query_map([], |row| row.get(0)).unwrap()
        .filter_map(Result::ok)
        .collect();

    for (i, office) in office_names.iter().enumerate() {
        println!("{}. {}", i + 1, office);
    }

    let office_choice = get_input("Type the name of the office you want to vote for:");
    if office_choice.to_lowercase() == "logout" {
        return Some("logout".to_string());
    }
    
    let office_choice_lower = office_choice.to_lowercase();
    for office in &office_names {
        if office.to_lowercase() == office_choice_lower {
            return Some(office.clone());
        }
    }
    println!("Invalid office choice.");
    None
}


// -----------------> Candidate choice function

fn choose_candidate(conn: &Connection, office_name: &str) -> Option<String> {
    println!("\n\n");
    println!("╔════════════════════════════════════════════════╗");
    println!("║             🎖️ Candidates for {} 🎖️             ║", office_name);
    println!("╚════════════════════════════════════════════════╝");

    let mut stmt = conn.prepare("SELECT name FROM candidates WHERE office_id = (SELECT id FROM offices WHERE name = ?)").unwrap();
    let candidate_names: Vec<String> = stmt.query_map(params![office_name], |row| row.get(0)).unwrap()
        .filter_map(Result::ok)
        .collect();

    for (i, candidate) in candidate_names.iter().enumerate() {
        println!("{}. {}", i + 1, candidate);
    }

    let candidate_choice = get_input("Type the name of the candidate you want to vote for:");
    let candidate_choice_lower = candidate_choice.to_lowercase();
    for candidate in &candidate_names {
        if candidate.to_lowercase() == candidate_choice_lower {
            return Some(candidate.clone());
        }
    }
    println!("Invalid candidate choice.");
    None
}

//--------------------->  Cast vote function

fn cast_vote(conn: &Connection, voter_name: &str, office_name: &str, candidate_name: &str) {
    let voter_id = get_voter_id(conn, voter_name).expect("Failed to retrieve voter ID.");
    let office_id = get_office_id(conn, office_name).expect("Failed to retrieve office ID.");

    // Retrieve voter_id from the voter_name
    let voter_id = get_voter_id(conn, voter_name).expect("Failed to retrieve voter ID.");

    // Use the voter_id to get the birth year
    let birth_year = get_voter_birth_year(conn, voter_id).expect("Failed to retrieve birth year.");
    let promo_eligible = birth_year > chrono::Utc::now().year();

    // Check if the voter has already voted for this office
    if !promo_eligible && db::has_voted(conn, voter_id, office_id, promo_eligible).expect("Failed to check voting status.") {
        println!("You have already voted for the {} office. Only one vote per office is allowed.", office_name);
    } else {
        // Cast vote for candidate if not already voted for this office
        db::cast_vote(conn, candidate_name).expect("Failed to cast vote.");
        db::record_vote(conn, voter_id, office_id).expect("Failed to record vote.");
        println!("Vote successfully cast for {} in the {} office.", candidate_name, office_name);
    }
}

// -------------> Function to retrieve voter ID
fn get_voter_id(conn: &Connection, name: &str) -> Result<i32, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id FROM voters WHERE name = ?1")?;
    stmt.query_row(params![name], |row| row.get(0))
}

//---------------> Function to retrieve office ID
fn get_office_id(conn: &Connection, name: &str) -> Result<i32, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id FROM offices WHERE name = ?1")?;
    stmt.query_row(params![name], |row| row.get(0))
}

// -------------> Function to retrieve input from the user
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() // Ensure the input is always lowercase
}



//                        =========================== POST ELECTION FUNCTIONS ==============================


// ------------> Tally votes function using the database
fn tally_vote(conn: &Connection) {
    println!("---------------- ELECTION RESULTS --------------\n");
    let mut stmt = conn.prepare(
        "SELECT o.name AS office_name, c.name AS candidate_name, c.party, c.votes 
         FROM offices o 
         JOIN candidates c ON c.office_id = o.id 
         ORDER BY o.name, c.votes DESC",
    ).expect("Failed to prepare tally query");

    let results = stmt.query_map([], |row| {
        let office_name: String = row.get(0)?;
        let candidate_name: String = row.get(1)?;
        let party: String = row.get(2)?;
        let votes: i32 = row.get(3)?;
        Ok((office_name, candidate_name, party, votes))
    }).expect("Failed to map query results");

    for result in results {
        let (office_name, candidate_name, party, votes) = result.expect("Failed to read row");
        println!("Office: {}\n  {} ({}) - {} votes", office_name, candidate_name, party, votes);
    }
}

//                      ============================== DELETION FUNCTIONS ===================================

// ----------> Delete a voter
fn delete_voter(conn: &Connection) {
    println!("\nType 'exit' at any time to cancel and return to the admin menu.");
    let voter_name = get_input("\n\tEnter the name of the voter to delete:");
    if voter_name.to_lowercase() == "exit" {
        println!("Deletion canceled. Returning to admin menu...");
        return;
    }

    match conn.execute(
        "DELETE FROM voters WHERE LOWER(name) = ?1",
        params![voter_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tVoter '{}' deleted successfully!", voter_name);
            } else {
                println!("\n\tNo voter found with that name.");
            }
        }
        Err(err) => println!("Failed to delete voter: {}", err),
    }
}

// ------------> Delete a candidate
fn delete_candidate(conn: &Connection) {
    println!("\nType 'exit' at any time to cancel and return to the admin menu.");
    let candidate_name = get_input("\n\tEnter the name of the candidate to delete:");
    if candidate_name.to_lowercase() == "exit" {
        println!("Deletion canceled. Returning to admin menu...");
        return;
    }

    let office_name = get_input("\n\tEnter the office the candidate is running for:");
    if office_name.to_lowercase() == "exit" {
        println!("Deletion canceled. Returning to admin menu...");
        return;
    }

    match conn.execute(
        "DELETE FROM candidates WHERE LOWER(name) = ?1 AND office_id = (SELECT id FROM offices WHERE LOWER(name) = ?2)",
        params![candidate_name, office_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tCandidate '{}' from office '{}' deleted successfully!", candidate_name, office_name);
            } else {
                println!("\n\tNo candidate found with that name and office.");
            }
        }
        Err(err) => println!("Failed to delete candidate: {}", err),
    }
}

// ---------------> Delete an office
fn delete_office(conn: &Connection) {
    println!("\nType 'exit' at any time to cancel and return to the admin menu.");
    let office_name = get_input("\n\tEnter the name of the office to delete:");
    if office_name.to_lowercase() == "exit" {
        println!("Deletion canceled. Returning to admin menu...");
        return;
    }

    // First, delete all candidates associated with this office
    match conn.execute(
        "DELETE FROM candidates WHERE office_id = (SELECT id FROM offices WHERE LOWER(name) = ?1)",
        params![office_name],
    ) {
        Ok(deleted) => {
            println!("\n\tDeleted {} candidates associated with office '{}'.", deleted, office_name);
        }
        Err(err) => println!("Failed to delete candidates: {}", err),
    }

    // Then, delete the office itself
    match conn.execute(
        "DELETE FROM offices WHERE LOWER(name) = ?1",
        params![office_name],
    ) {
        Ok(deleted) => {
            if deleted > 0 {
                println!("\n\tOffice '{}' deleted successfully!", office_name);
            } else {
                println!("\n\tNo office found with that name.");
            }
        }
        Err(err) => println!("Failed to delete office: {}", err),
    }
}

//                              ========================== EOF ====================