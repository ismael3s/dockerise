pub struct DockerFilePath {
    cs_proj: String,
    cs_proj_folder: String,
}

impl DockerFilePath {
    pub fn new(
        system_path: &str,
        root_path: &str,
        main_separator: char,
    ) -> Result<DockerFilePath, &'static str> {
        let path_segments = system_path
            .split(root_path)
            .last()
            .unwrap_or("")
            .split(main_separator)
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let cs_proj_folder = path_segments
            .iter()
            .filter(|p| !p.contains("csproj"))
            .map(|file| file.to_string())
            .collect::<Vec<_>>()
            .join("/");
        let cs_proj = path_segments.join("/");
        Ok(DockerFilePath {
            cs_proj,
            cs_proj_folder,
        })
    }
    pub fn dll(&self) -> String {
        self.cs_proj
            .split("/")
            .last()
            .unwrap()
            .replace("csproj", "dll")
    }
}

pub struct DockerfileBuilder {
    dockerfile: String,
}

impl DockerfileBuilder {
    pub fn new() -> DockerfileBuilder {
        DockerfileBuilder {
            dockerfile: String::new(),
        }
    }

    pub fn copy(&mut self, command: &str) -> &mut DockerfileBuilder {
        self.dockerfile.push_str(&format!("COPY {}\n", command));
        self
    }

    pub fn run(&mut self, command: &str) -> &mut DockerfileBuilder {
        self.dockerfile.push_str(&format!("RUN {}\n", command));
        self
    }

    pub fn entrypoint(&mut self, command: &str) -> &mut DockerfileBuilder {
        self.dockerfile
            .push_str(&format!("ENTRYPOINT {}\n", command));
        self
    }

    // pub fn expose(&mut self, port: &str) -> &mut DockerfileBuilder {
    //     self.dockerfile.push_str(&format!("EXPOSE {}\n", port));
    //     self
    // }

    pub fn from(&mut self, port: &str) -> &mut DockerfileBuilder {
        self.dockerfile.push_str(&format!("FROM {}\n", port));
        self
    }

    pub fn workdir(&mut self, workdir: &str) -> &mut DockerfileBuilder {
        self.dockerfile.push_str(&format!("WORKDIR {}\n", workdir));
        self
    }

    pub fn build(&self) -> String {
        self.dockerfile.clone()
    }
}

pub mod dotnet {
    use super::{DockerFilePath, DockerfileBuilder};

    pub struct Builder {
        docker_file_builder: DockerfileBuilder,
        copy_csproj: String,
        copy_project: String,
    }
    pub fn new() -> Builder {
        Builder {
            docker_file_builder: DockerfileBuilder::new(),
            copy_csproj: String::new(),
            copy_project: String::new(),
        }
    }

    impl Builder {
        pub fn dotnet(&mut self, dotnet_version: &str) -> &mut Builder {
            self.docker_file_builder.dockerfile = format!(
                "
FROM mcr.microsoft.com/dotnet/aspnet:{dotnet_version} AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/sdk:{dotnet_version} AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
",
                dotnet_version = dotnet_version
            );
            return self;
        }

        pub fn copy_csproj(&mut self, docker_file_path: &DockerFilePath) -> &mut Builder {
            self.copy_csproj.push_str(&format!(
                "COPY ./{} ./{}\n",
                docker_file_path.cs_proj, docker_file_path.cs_proj
            ));
            return self;
        }

        pub fn copy_csproj_folder(&mut self, docker_file_path: &DockerFilePath) -> &mut Builder {
            self.copy_project.push_str(&format!(
                "COPY ./{} ./{}\n",
                docker_file_path.cs_proj_folder, docker_file_path.cs_proj_folder
            ));
            return self;
        }

        pub fn build(&mut self, startup_docker_file_path: &DockerFilePath) -> String {
            self.docker_file_builder
                .dockerfile
                .push_str(&self.copy_csproj);
            self.docker_file_builder.dockerfile.push_str(&format!(
                "RUN dotnet restore ./{}\n",
                startup_docker_file_path.cs_proj
            ));
            self.docker_file_builder
                .dockerfile
                .push_str(&self.copy_project);
            self.docker_file_builder.dockerfile.push_str(&format!(
                "RUN dotnet build ./{} -c $BUILD_CONFIGURATION -o /app/build\n",
                startup_docker_file_path.cs_proj
            ));
            self.docker_file_builder.dockerfile.push_str(&format!(
                "
FROM build as publish
RUN dotnet publish ./{} -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT [\"dotnet\", \"{}\"]
",
                startup_docker_file_path.cs_proj,
                startup_docker_file_path.dll()
            ));
            return self.docker_file_builder.dockerfile.clone();
        }
    }
}

pub mod frontend {
    use super::DockerfileBuilder;

    pub struct Builder {
        docker_file_builder: DockerfileBuilder,
        node_version: String,
        is_nextjs: bool,
    }

    pub fn new() -> Builder {
        Builder {
            docker_file_builder: DockerfileBuilder::new(),
            node_version: String::from("18.0"),
            is_nextjs: false,
        }
    }

    impl Builder {
        pub fn with_node_version(&mut self, node_version: &str) -> &mut Builder {
            self.node_version = node_version.to_string();
            self
        }

        pub fn is_nextjs(&mut self) -> &mut Builder {
            self.is_nextjs = true;
            self
        }

        pub fn build(&mut self) -> String {
            let dist_folder = if self.is_nextjs { ".next" } else { "dist" };
            return self
                .docker_file_builder
                .from(&format!("node:{} AS build", self.node_version))
                .workdir("/app")
                .copy("package*.json ./")
                .copy(". .")
                .run("npm run build")
                .from(&format!("node:{} AS production", self.node_version))
                .workdir("/app")
                .copy("--from=build /app/package*.json ./")
                .run("npm ci")
                .copy(
                    format!(
                        "--from=build /app/{dist_folder} ./{dist_folder}",
                        dist_folder = dist_folder
                    )
                    .as_str(),
                )
                .copy("--from=build /app/public ./public")
                .entrypoint("[\"npm\", \"start\"]")
                .build();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DockerFilePath;

    #[test]
    fn docker_file_path_should_be_able_to_parse_windows_cs_project() {
        let sut = DockerFilePath::new(
            "C:\\Users\\user\\sln_folder\\project_folder\\project_folder.csproj",
            "C:\\Users\\user\\sln_folder",
            '\\',
        );

        assert_eq!(sut.is_ok(), true);
        let result = sut.unwrap();
        assert_eq!(result.cs_proj, "project_folder/project_folder.csproj");
        assert_eq!(result.cs_proj_folder, "project_folder");
    }

    #[test]
    fn docker_file_path_should_be_able_to_parse_linx_cs_project() {
        let sut = DockerFilePath::new(
            "/home/user/sln_folder/project_folder/project_folder.csproj",
            "/home/user/sln_folder",
            '/',
        );
        assert_eq!(sut.is_ok(), true);
        let result = sut.unwrap();
        assert_eq!(result.cs_proj, "project_folder/project_folder.csproj");
        assert_eq!(result.cs_proj_folder, "project_folder");
    }

    #[test]
    fn dockerfile_path_should_be_able_to_parse_projects_inside_nested_folder() {
        let sut = DockerFilePath::new(
            "/home/user/sln_folder/src/project_folder/project_folder.csproj",
            "/home/user/sln_folder",
            '/',
        );
        assert_eq!(sut.is_ok(), true);
        let result = sut.unwrap();
        assert_eq!(result.cs_proj, "src/project_folder/project_folder.csproj");
        assert_eq!(result.cs_proj_folder, "src/project_folder");
    }
}
