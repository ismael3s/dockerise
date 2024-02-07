// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dockerfile_builder;
mod parse_dotnet_projects_reference;
mod write_dockerfile;
use std::{fs::File, io::BufReader, path::MAIN_SEPARATOR};
use tauri::{Manager, Window};
use walkdir::WalkDir;

use crate::{dockerfile_builder::DockerFilePath, parse_dotnet_projects_reference::Mermaid};

use parse_dotnet_projects_reference::Project;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
enum ProjectType {
    Dotnet,
    Next,
    Frontend,
}

#[derive(Debug, Serialize)]
struct Output {
    dockerfile: String,
    mermaid: Option<String>,
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
fn write_docker(project_root: String, dockerfile: String) -> Result<(), &'static str> {
    let input = write_dockerfile::Input {
        project_root,
        dockerfile,
        should_override: true,
    };
    return write_dockerfile::WriteDockerfile::new().execute(input);
}
#[tauri::command]
fn my_custom_command(
    project_root: String,
    project_type: ProjectType,
    maybe_startup_project: Option<String>,
) -> Output {
    match project_type {
        ProjectType::Dotnet => {
            let startup_project = maybe_startup_project.unwrap_or("".to_string());
            return dotnet(&project_root, &startup_project);
        }
        ProjectType::Next => {
            return frontend_nextjs();
        }
        ProjectType::Frontend => {
            return frontend();
        }
    };
}
#[tauri::command]
async fn close_splashscreen(window: Window) {
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
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

fn dotnet(project_root: &str, startup_project: &str) -> Output {
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

    return Output {
        dockerfile,
        mermaid: Some(projects.to_mermaid()),
    };
}

fn frontend() -> Output {
    // TODO: the node version should be dynamic, validate the version from the package.json
    let docker_file = dockerfile_builder::frontend::new()
        .with_node_version("18.17-alpine")
        .build();
    return Output {
        dockerfile: docker_file,
        mermaid: None,
    };
}

fn frontend_nextjs() -> Output {
    // TODO: the node version should be dynamic, validate the version from the package.json
    let docker_file = dockerfile_builder::frontend::new()
        .with_node_version("18.17-alpine")
        .is_nextjs()
        .build();
    return Output {
        dockerfile: docker_file,
        mermaid: None,
    };
}
