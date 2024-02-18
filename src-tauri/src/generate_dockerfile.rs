use std::{fs::File, io::BufReader, path::MAIN_SEPARATOR};

use serde::Serialize;

use crate::{
    commands::find_projects_files,
    dockerfile_builder::{self, DockerFilePath},
    parse_dotnet_projects_reference::{Mermaid, Project},
    ProjectType,
};

pub struct GenerateDockerfile {
    maybe_startup_project: Option<String>,
    project_type: ProjectType,
    project_root: String,
}

#[derive(Debug, Serialize)]
pub struct Output {
    dockerfile: String,
    mermaid: Option<String>,
}

impl GenerateDockerfile {
    pub fn new(
        project_root: String,
        project_type: ProjectType,
        startup_project: Option<String>,
    ) -> Self {
        Self {
            maybe_startup_project: startup_project,
            project_type,
            project_root,
        }
    }

    pub fn execute(&self) -> Output {
        match self.project_type {
            ProjectType::Dotnet => {
                let startup_project = self.maybe_startup_project.clone().unwrap_or("".to_string());
                return dotnet(&self.project_root, &startup_project);
            }
            ProjectType::Next => {
                return frontend_nextjs();
            }
            ProjectType::Vite => {
                return frontend_vite();
            }
        };
    }
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

fn frontend_nextjs() -> Output {
    let docker_file = dockerfile_builder::DockerfileBuilder::new()
        .from(&format!("node:{} AS build", "18-alpine"))
        .workdir("/app")
        .copy("package*.json ./")
        .run("npm ci")
        .copy(". .")
        .run("npm run build")
        .from(&format!("node:{} AS production", "18-alpine"))
        .workdir("/app")
        .copy("--from=build /app/package*.json ./")
        .run("npm ci")
        .copy("--from=build /app/.next ./.next")
        .copy("--from=build /app/public ./public")
        .entrypoint("[\"npm\", \"start\"]")
        .build();
    return Output {
        dockerfile: docker_file,
        mermaid: None,
    };
}

fn frontend_vite() -> Output {
    let docker_file = dockerfile_builder::DockerfileBuilder::new()
        .from(&format!("node:{} AS build", "18-alpine"))
        .workdir("/build")
        .copy("package*.json ./")
        .run("npm ci")
        .copy(". .")
        .run("npm run build")
        .from("nginx:alpine AS production")
        .copy("--from=build /build/dist /usr/share/nginx/html")
        .build();
    return Output {
        dockerfile: docker_file,
        mermaid: None,
    };
}
