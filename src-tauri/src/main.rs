// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod dockerfile_builder;
mod generate_dockerfile;
mod parse_dotnet_projects_reference;
mod write_dockerfile;
use crate::commands::{close_splashscreen, find_projects_files, write_docker};

use generate_dockerfile::{GenerateDockerfile, Output};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ProjectType {
    Dotnet,
    Next,
    Vite,
}

#[tauri::command]
fn my_custom_command(
    project_root: String,
    project_type: ProjectType,
    maybe_startup_project: Option<String>,
) -> Output {
    return GenerateDockerfile::new(project_root, project_type, maybe_startup_project).execute();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            find_projects_files,
            write_docker,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
