// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dockerfile_builder;
mod parse_dotnet_projects_reference;

use std::{fs::File, io::BufReader, path::MAIN_SEPARATOR};
use walkdir::WalkDir;

use crate::{dockerfile_builder::DockerFilePath, parse_dotnet_projects_reference::Mermaid};

use parse_dotnet_projects_reference::Project;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Output {
    dockerfile: String,
    mermaid: String,
}

#[tauri::command]
fn find_projects_files(path: &str) -> Vec<String> {
    return WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| !e.path().display().to_string().contains("test"))
        .filter_map(|e| e.ok())
        .filter(|file| matches!(file.path().extension(), Some(extension) if extension == "csproj"))
        .map(|file| file.path().to_str().unwrap().to_string())
        .collect::<_>();
}

#[tauri::command]
fn my_custom_command(project_root: String, startup_project: String) -> Output {
    let mut projects: Vec<Project> = Vec::new();
    let mut docker_file_builder = dockerfile_builder::dotnet::new();
    let startup_docker_file_path =
        DockerFilePath::new(&startup_project, &project_root, MAIN_SEPARATOR).unwrap();
    find_projects_files(&project_root)
        .iter()
        .for_each(|filename_with_path| {
            let file = BufReader::new(File::open(&filename_with_path).unwrap());
            let mut project: Project = quick_xml::de::from_reader(file).unwrap();
            project.update_items_groups();
            project.update_project_name(&filename_with_path);
            projects.push(project);
            // TODO: need to handle err causes, because the dockerfile will be invalid when trying to build.
            match DockerFilePath::new(&filename_with_path, &project_root, MAIN_SEPARATOR) {
                Ok(docker_file_path) => {
                    docker_file_builder
                        .copy_csproj(&docker_file_path)
                        .copy_csproj_folder(&docker_file_path);
                }
                Err(_) => {}
            }
        });
    let dotnet_version = projects.first().unwrap().get_dotnet_version();
    let dockerfile = docker_file_builder
        .dotnet(&dotnet_version)
        .build(&startup_docker_file_path);
    println!("{}", projects.to_mermaid());

    return Output {
        dockerfile,
        mermaid: projects.to_mermaid(),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            find_projects_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
