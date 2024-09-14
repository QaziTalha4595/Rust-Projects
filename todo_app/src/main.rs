use rusqlite::{params, Connection, Result};
use dirs::home_dir;
use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}


fn get_db_path() -> PathBuf {
    // Get the user's home directory
    let mut path = home_dir().expect("Unable to find user home directory");

    // Add the .todo_app directory and tasks.db file
    path.push(".todo_app");
    
    // Create the directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(&path).expect("Unable to create .todo_app directory");
    }

    // Append the database file to the path
    path.push("tasks.db");

    path
}


// Initialize the SQLite database (create table if not exists)
fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
             id          INTEGER PRIMARY KEY,
             description TEXT NOT NULL,
             completed   BOOLEAN NOT NULL
         )",
        [],
    )?;

    Ok(conn)
}

// Add a new task
fn add_task(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        params![description, false],
    )?;
    Ok(())
}

// List all tasks
fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

// Mark a task as completed
fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET completed = 1 WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

// Delete a task
fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}

// Main function for the to-do app
fn main() -> Result<()> {
    let conn = init_db()?;

    // Example of adding, listing, and completing tasks
    add_task(&conn, "Learn Rust")?;
    add_task(&conn, "Build a to-do app")?;

    let tasks = list_tasks(&conn)?;
    println!("Tasks:");
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.description, if task.completed { "x" } else { " " });
    }

    complete_task(&conn, 1)?;

    let tasks = list_tasks(&conn)?;
    println!("Updated Tasks:");
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.description, if task.completed { "x" } else { " " });
    }

    Ok(())
}
