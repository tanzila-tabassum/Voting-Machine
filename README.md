# 🗳️ Voting Machine

A command-line voting system built in **Rust**, backed by a **SQLite** database. Developed as part of a Secure Systems Engineering course (EE G7701), this project implements a fully functional election management system with admin and voter roles, ballot management, and vote tallying.

**Authors:** Maïmouna Traoré · Tanzila Tabassum · Bryan Mgbeojirikwe

---

## 📖 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Database Schema](#database-schema)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation & Running](#installation--running)
- [Usage](#usage)
  - [Admin](#admin)
  - [Voter](#voter)
- [Project Structure](#project-structure)
- [Testing](#testing)
- [Security Analysis](#security-analysis)
- [Contributing](#contributing)
- [License](#license)

---

## Features

**Admin functionality:**
- 🔐 Secure admin login with username and password
- 🏛️ Create elections with multiple offices and candidates
- 📋 Register new voters (name + date of birth)
- 🟢 Open and close ballots for voting
- 📊 Tally and display election results
- 🗑️ Delete voters, candidates, and offices

**Voter functionality:**
- 🪪 Voter login with name and date of birth verification
- 🗳️ Cast votes for candidates across multiple offices
- 🔒 One vote per voter enforcement

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| [Rust](https://www.rust-lang.org/) | Core application language |
| [SQLite](https://www.sqlite.org/) | Persistent data storage via `voting_machine.db` |
| [Cargo](https://doc.rust-lang.org/cargo/) | Build system and dependency management |

---

## Database Schema

The system uses four SQLite tables:

| Table | Columns | Description |
|-------|---------|-------------|
| `voters` | `id`, `name`, `date_of_birth`, `has_voted` | Registered voter records |
| `candidates` | `id`, `name`, `party`, `votes`, `office_id` | Candidates and their vote counts |
| `offices` | `id`, `name` | Election offices (e.g. President, Mayor, Judge) |
| `votes` | `id`, `voter_id`, `office_id` | Vote records linking voters to offices |

---

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) installed on your system
- Works on all major operating systems (Linux, macOS, Windows, or inside a VM)
- Optionally, install [SQLite Browser](https://www.sqlite.org/download.html) to inspect `voting_machine.db` directly

### Installation & Running

1. **Clone or download the repository:**
   ```bash
   git clone https://github.com/tanzila-tabassum/Voting-Machine.git
   cd Voting-Machine/voting_machine
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the project:**
   ```bash
   cargo run
   ```

   You will see the following welcome screen:
   ```
   ██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗
   WELCOME To the Voting Machine
   Are you an admin, or a voter? (Type 'exit' to leave)
   ```

---

## Usage

### Admin

1. Type `admin` at the main prompt and press Enter.
2. Enter credentials:
   - Username: `adminname`
   - Password: `adminpassword`
3. Once logged in, you will see the **Admin Menu**:

```
⚙️  Admin Menu  ⚙️
1. Create a New Election
2. Register a New Voter
3. Open Election for Voting
4. Close Election to End Voting
5. Tally Votes
6. Delete a Voter
7. Delete a Candidate
8. Delete an Office
9. Exit Admin Menu
```

Enter a number to select an option. Type `exit` at any prompt to cancel and return to the Admin Menu.

**Creating an election (option 1):** Enter an office name (e.g. `President`, `Judge`, `Mayor`), then add candidates with their name and political party. Type `yes` to add more candidates or offices, `no` when done.

**Registering a voter (option 2):** Enter the voter's full name and date of birth (MM/DD/YYYY). Confirm the details when prompted.

**Tallying votes (option 5):** Displays vote counts for all candidates grouped by office. Only available after closing the ballot.

---

### Voter

1. Type `voter` at the main prompt and press Enter.
2. Enter your registered name and date of birth (MM/DD/YYYY).
3. Once verified, you'll see the available offices. Type the name of the office you want to vote for.
4. Select your candidate by typing their name.
5. A confirmation message will confirm your vote was cast.
6. Type `logout` to return to the main menu.

> ⚠️ The election must be **opened** by an admin before voters can cast votes.

---

## Project Structure

```
Voting-Machine/
├── voting_machine/
│   ├── src/
│   │   ├── main.rs          # Entry point, CLI menus, admin & voter flows
│   │   ├── db.rs            # Database setup and all SQLite queries
│   │   └── lib.rs           # Shared library functions and utilities
│   ├── tests/
│   │   ├── db_tests.rs      # Database unit and integration tests
│   │   └── main_tests.rs    # Application logic tests
│   ├── libs/                # External/helper libraries
│   ├── build.rs             # Cargo build script
│   ├── Cargo.toml           # Dependencies and project metadata
│   ├── Cargo.lock           # Locked dependency versions
│   ├── voting_machine.db    # SQLite database file
│   └── User_Manual_Voting_System.docx.pdf  # Full user manual
└── LICENSE
```

---

## Testing

The project includes a comprehensive test suite covering core functionality, edge cases, and ballot state management.

Run all tests with:
```bash
cargo test
```

**Test coverage includes:**

- `test_register_voter` — voter registration and database insertion
- `test_register_duplicate_voter` — duplicate registration prevention
- `test_has_voted` — vote tracking and prevention of double voting
- `test_verify_voter_invalid_date` — invalid date of birth handling
- `test_open_ballot_already_open` — idempotent ballot open behavior
- `test_close_ballot_already_closed` — idempotent ballot close behavior

These tests ensure code reliability, validate edge cases, support safe refactoring, and demonstrate compliance with project requirements.

---

## Security Analysis

This project was subjected to a **red team security analysis** as part of the course. The following vulnerabilities were identified:

### Backdoor #1 — Age Verification Bypass
The `verify_voter` function uses `year_threshold = current_year + 2` instead of `current_year`. Any voter born after the threshold year bypasses normal age validation and has their `has_voted` flag silently reset to `0`, allowing them to vote multiple times.

### Backdoor #2 — Silent Test Voter with Vote Injection
The `register_our_voter` function secretly creates a test voter during registration. If the last character of the voter's name is a digit (1–9), the system disables foreign key constraints (`PRAGMA foreign_keys = OFF`) and casts a vote for the candidate matching that digit ID — then deletes the test voter. This allows manipulating vote tallies silently.

### Additional Security Issues

| Vulnerability | Description |
|--------------|-------------|
| **Registration Manipulation** | Anyone with the `adminname` username can register unlimited voters without proper access control |
| **Vote Tally Manipulation** | Admin access requires only a username — no password validation in some flows — allowing fake votes to be added directly |
| **Candidate Data Volatility** | Candidate data exists only in memory and is lost on program exit, enabling manipulation via election restart |
| **CSV File Vulnerability** | Voter data read/written to `voters.csv` has no integrity validation, allowing external tampering and duplicate registrations |

> 📄 See `User_Manual_Voting_System.docx.pdf` for the full system documentation.

---

## Contributing

Contributions are welcome! To get started:

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m "Add your feature"`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

Please ensure your code follows Rust conventions and includes relevant tests.

---

## License

This project is licensed under the terms found in the [LICENSE](../LICENSE) file.

---

> Developed for Secure Systems Engineering (EE G7701) — November 2024.
