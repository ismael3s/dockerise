use tauri::{Manager, Window};
use walkdir::WalkDir;

use crate::{write_dockerfile, ProjectType};

#[tauri::command]
pub async fn close_splashscreen(window: Window) {
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

#[tauri::command]
pub fn write_docker(
    project_root: String,
    dockerfile: String,
    project_type: ProjectType,
) -> Result<(), &'static str> {
    let input = write_dockerfile::Input {
        project_root,
        dockerfile,
        project_type,
        should_override: true,
    };
    return write_dockerfile::WriteDockerfile::new().execute(input);
}

#[tauri::command]
pub fn find_projects_files(path: &str) -> Vec<String> {
    return WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| !e.path().display().to_string().contains("test"))
        .filter_map(|e| e.ok())
        .filter(|file| matches!(file.path().extension(), Some(extension) if extension == "csproj"))
        .map(|file| file.path().to_str().unwrap().to_string())
        .collect::<_>();
}
