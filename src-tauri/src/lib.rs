use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{Manager, State};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project {
    name: String,
    #[serde(rename = "type")]
    project_type: String,
    path: String,
    command: String,
    running: bool,
    output: String,
}

struct AppState {
    processes: Mutex<HashMap<usize, Child>>,
    outputs: Arc<Mutex<HashMap<usize, String>>>,
}

#[tauri::command]
fn get_projects(app: tauri::AppHandle) -> Result<Vec<Project>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;

    let projects_file = data_dir.join("projects.json");

    if !projects_file.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(projects_file).map_err(|e| e.to_string())?;
    let projects: Vec<Project> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());

    Ok(projects)
}

#[tauri::command]
fn save_projects(app: tauri::AppHandle, projects: Vec<Project>) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;

    let projects_file = data_dir.join("projects.json");
    let content = serde_json::to_string_pretty(&projects).map_err(|e| e.to_string())?;

    std::fs::write(projects_file, content).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn start_project(
    state: State<AppState>,
    path: String,
    command: String,
    index: usize,
) -> Result<(), String> {
    if command.is_empty() {
        return Err("Empty command".to_string());
    }

    // Determine shell and flags based on OS
    let mut child = if cfg!(target_os = "windows") {
        // Windows: use cmd
        Command::new("cmd")
            .arg("/C")
            .arg(&command)
            .current_dir(&path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start process: {}", e))?
    } else {
        // Unix: use bash with interactive login shell to properly load PATH
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        #[cfg(unix)]
        {
            // Source the user's profile and then run the command
            // This ensures npm, node, and other tools are available
            let wrapped_command = format!(
                "source ~/.profile 2>/dev/null || true; source ~/.bashrc 2>/dev/null || true; source ~/.zshrc 2>/dev/null || true; {}",
                command
            );

            Command::new(&shell)
                .arg("-c")  // command mode
                .arg(&wrapped_command)
                .current_dir(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .process_group(0)  // Create new process group
                .spawn()
                .map_err(|e| format!("Failed to start process with {}: {}", shell, e))?
        }

        #[cfg(not(unix))]
        {
            let wrapped_command = format!(
                "source ~/.profile 2>/dev/null || true; source ~/.bashrc 2>/dev/null || true; {}",
                command
            );

            Command::new(&shell)
                .arg("-c")
                .arg(&wrapped_command)
                .current_dir(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start process with {}: {}", shell, e))?
        }
    };

    // Take stdout and stderr to read in background threads
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    // Store the process
    let mut processes = state.processes.lock().unwrap();
    processes.insert(index, child);

    // Initialize output
    let mut outputs = state.outputs.lock().unwrap();
    outputs.insert(index, String::new());
    drop(outputs); // Release lock before spawning threads

    // Spawn thread to read stdout
    if let Some(stdout) = stdout {
        let outputs_clone = state.outputs.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut outputs = outputs_clone.lock().unwrap();
                    if let Some(output) = outputs.get_mut(&index) {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
            }
        });
    }

    // Spawn thread to read stderr
    if let Some(stderr) = stderr {
        let outputs_clone = state.outputs.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut outputs = outputs_clone.lock().unwrap();
                    if let Some(output) = outputs.get_mut(&index) {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
            }
        });
    }

    Ok(())
}

#[tauri::command]
fn stop_project(state: State<AppState>, index: usize) -> Result<(), String> {
    let mut processes = state.processes.lock().unwrap();

    if let Some(mut child) = processes.remove(&index) {
        #[cfg(unix)]
        {
            // On Unix, kill the entire process group to ensure all child processes are terminated
            let pid = child.id() as i32;
            unsafe {
                // First try SIGTERM for graceful shutdown
                libc::kill(-pid, libc::SIGTERM);
                // Give it a moment
                std::thread::sleep(std::time::Duration::from_millis(100));
                // Then send SIGKILL to force kill any remaining processes
                libc::kill(-pid, libc::SIGKILL);
            }
        }

        #[cfg(not(unix))]
        {
            // On Windows, just kill the process normally
            child.kill().map_err(|e| format!("Failed to kill process: {}", e))?;
        }

        child.wait().map_err(|e| format!("Failed to wait for process: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn get_project_output(state: State<AppState>, index: usize) -> Result<String, String> {
    let outputs = state.outputs.lock().unwrap();

    if let Some(output) = outputs.get(&index) {
        Ok(output.clone())
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
fn detect_package_manager(path: String) -> Result<String, String> {
    let path_buf = std::path::PathBuf::from(&path);

    // Check for yarn.lock first
    if path_buf.join("yarn.lock").exists() {
        return Ok("yarn".to_string());
    }

    // Check for package-lock.json
    if path_buf.join("package-lock.json").exists() {
        return Ok("npm".to_string());
    }

    // Default to npm if no lock file found
    Ok("npm".to_string())
}

#[tauri::command]
fn install_packages(
    state: State<AppState>,
    path: String,
    package_manager: String,
    index: usize,
) -> Result<(), String> {
    let command = match package_manager.as_str() {
        "yarn" => "yarn install",
        "npm" => "npm install",
        _ => return Err("Invalid package manager".to_string()),
    };

    // Use the same logic as start_project to run the install command
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .current_dir(&path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start install: {}", e))?
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        #[cfg(unix)]
        {
            let wrapped_command = format!(
                "source ~/.profile 2>/dev/null || true; source ~/.bashrc 2>/dev/null || true; source ~/.zshrc 2>/dev/null || true; {}",
                command
            );

            Command::new(&shell)
                .arg("-c")
                .arg(&wrapped_command)
                .current_dir(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .process_group(0)
                .spawn()
                .map_err(|e| format!("Failed to start install with {}: {}", shell, e))?
        }

        #[cfg(not(unix))]
        {
            let wrapped_command = format!(
                "source ~/.profile 2>/dev/null || true; source ~/.bashrc 2>/dev/null || true; {}",
                command
            );

            Command::new(&shell)
                .arg("-c")
                .arg(&wrapped_command)
                .current_dir(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start install with {}: {}", shell, e))?
        }
    };

    // Take stdout and stderr to read in background threads
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    // Store the process
    let mut processes = state.processes.lock().unwrap();
    processes.insert(index, child);

    // Initialize/append to output
    let mut outputs = state.outputs.lock().unwrap();
    let output_str = outputs.entry(index).or_insert_with(String::new);
    output_str.push_str(&format!("Running {} install...\n", package_manager));
    drop(outputs);

    // Spawn thread to read stdout
    if let Some(stdout) = stdout {
        let outputs_clone = state.outputs.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut outputs = outputs_clone.lock().unwrap();
                    if let Some(output) = outputs.get_mut(&index) {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
            }
        });
    }

    // Spawn thread to read stderr
    if let Some(stderr) = stderr {
        let outputs_clone = state.outputs.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut outputs = outputs_clone.lock().unwrap();
                    if let Some(output) = outputs.get_mut(&index) {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
            }
        });
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            processes: Mutex::new(HashMap::new()),
            outputs: Arc::new(Mutex::new(HashMap::new())),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_projects,
            save_projects,
            start_project,
            stop_project,
            get_project_output,
            detect_package_manager,
            install_packages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
