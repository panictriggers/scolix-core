use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn FirstSetup() -> Result<()> {
    let conn = Connection::open("../data/core.db").unwrap();

    // USER table
    conn.execute(
        "create table if not exists Users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            pwd_hash TEXT NOT NULL,
            pwd_salt TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            telnr TEXT NULL,
            dob DATE NOT NULL,
            address TEXT NULL,
            postalcode TEXT NULL,
            city TEXT NULL,
            medical TEXT NOT NULL,
            studentID INTEGER NULL references Students(id),
            teacherID INTEGER NULL references Teachers(id)
         )",
        NO_PARAMS,
    ).unwrap();

    // STUDENTS table
    conn.execute(
        "create table if not exists Students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            classIDs INTEGER NOT NULL references Classes(id),
            difficulty INTEGER NOT NULL references Difficulty(id),
            mentorIDs INTEGER NOT NULL references Teachers(id),
            profile TEXT NOT NULL,
            scheduleref TEXT NULL,
            gradesref TEXT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    // TEACHERS table
    conn.execute(
        "create table if not exists Teachers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            abr TEXT NOT NULL,
            subject INTEGER NOT NULL references Subjects(id),
            mentorclass INTEGER NOT NULL references Classes(id) 
        )",
        NO_PARAMS,
    ).unwrap();

    // DIFFICULTY table
    conn.execute(
        "create table if not exists Difficulty (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    // SUBJECTS table
    conn.execute(
        "create table if not exists Subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    // CLASSES table
    conn.execute(
        "create table if not exists Classes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            abr TEXT NOT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    // EVENTS table
    conn.execute(
        "create table if not exists Events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            discription TEXT NULL,
            homework TEXT NULL,
            type_exam INTEGER NULL references TypeExams(id),
            members TEXT NULL,
            classID TEXT NULL,
            teachersID TEXT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    // TYPE_EXAM
    conn.execute(
        "create table if not exists TypeExam (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
         )",
        NO_PARAMS,
    ).unwrap();

    Ok(())
}