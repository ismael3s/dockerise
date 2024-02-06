use std::{fs::File, io::Write};

pub struct Input {
    pub project_root: String,
    pub dockerfile: String,
    pub should_override: bool,
}

pub struct WriteDockerfile {}

impl WriteDockerfile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, input: Input) -> Result<(), &'static str> {
        let file_path = format!("{}/Dockerfile", input.project_root);
        let file = File::create(&file_path);
        if file.is_err() {
            return Err("Houve um erro ao criar Dockerfile");
        }
        let file_write_result = file.unwrap().write_all(input.dockerfile.as_bytes());
        if file_write_result.is_err() {
            return Err("Erro ao escrever Dockerfile");
        }

        let dockerignore = "
node_modules/
dist/
.next/
**/node_modules
**/dist
**/.next        
**/.classpath
**/.dockerignore
**/.env
**/.git
**/.gitignore
**/.project
**/.settings
**/.toolstarget
**/.vs
**/.vscode
**/*.*proj.user
**/*.dbmdl
**/*.jfm
**/azds.yaml
**/bin
**/charts
**/docker-compose*
**/Dockerfile*
**/node_modules
**/npm-debug.log
**/obj
**/secrets.dev.yaml
**/values.dev.yaml
LICENSE
README.md
!**/.gitignore
!.git/HEAD
!.git/config
!.git/packed-refs
!.git/refs/heads/**
        ";

        let mut file = match File::create(format!("{}/.dockerignore", input.project_root)) {
            Ok(file) => file,
            Err(_) => return Err("Erro ao criar .dockerignore"),
        };
        match file.write_all(dockerignore.as_bytes()) {
            Err(_) => return Err("Erro ao escrever .dockerignore"),
            _ => {}
        }

        return Ok(());
    }
}
