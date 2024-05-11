use serde::{Deserialize, Serialize};
use std::path::MAIN_SEPARATOR;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertyGroup {
    #[serde(rename = "TargetFramework", default)]
    target_framework: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProjectReference {
    #[serde(rename = "@Include")]
    include: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ItemGroup {
    #[serde(rename = "ProjectReference", default)]
    project_reference: Vec<ProjectReference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    #[serde(rename = "Name", default)]
    name: String,
    #[serde(rename = "PropertyGroup")]
    property_group: Vec<PropertyGroup>,
    #[serde(rename = "ItemGroup", default)]
    items_groups: Vec<ItemGroup>,
}

impl ProjectReference {
    pub fn get_include(&self) -> String {
        return self.include.clone().split('\\').last().unwrap().to_string();
    }
}

impl Project {
    pub fn get_dotnet_version(&self) -> String {
        self.property_group
            .iter()
            .filter(|pg| pg.target_framework.len() > 0)
            .next()
            .unwrap()
            .target_framework
            .clone()
            .chars()
            .filter(|c| c.is_digit(10) || c == &'.')
            .collect()
    }

    pub fn update_project_name(&mut self, project_full_name: &str) {
        self.name = project_full_name
            .split(MAIN_SEPARATOR)
            .last()
            .unwrap()
            .to_string();
    }

    pub fn filter_only_item_groups_with_reference(&self) -> Vec<ItemGroup> {
        return self
            .items_groups
            .clone()
            .into_iter()
            .filter(|item_group| item_group.project_reference.len() > 0)
            .collect();
    }

    pub fn update_items_groups(&mut self) {
        let new_item_groups = self.filter_only_item_groups_with_reference();
        self.items_groups = new_item_groups;
    }
}

pub trait Mermaid {
    fn to_mermaid(&self) -> String;
}

impl Mermaid for Vec<Project> {
    fn to_mermaid(&self) -> String {
        let mut mermaid = String::new();
        mermaid.push_str("graph LR\n");
        mermaid.push_str("\tsubgraph Dependencias\n");
        mermaid.push_str("\t\tdirection LR\n");
        for project in self {
            for item_group in &project.items_groups {
                for project_reference in &item_group.project_reference {
                    mermaid.push_str(&format!(
                        "\t\t{} -->{}\n",
                        project.name,
                        project_reference.get_include()
                    ));
                }
            }
        }
        mermaid.push_str("\tend\n");
        return mermaid;
    }
}
