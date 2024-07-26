use std::path::PathBuf;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use crate::new::parse_file::create_project;

mod parse_file;
#[derive(Eq, PartialEq,Clone,Debug)]
pub enum NewType{
    ActixApp,
    AxumApp,
    NtexApp,
    DioxusSsr,
    DioxusDesktop,
    MinimalApp
}


#[derive(Clone)]
pub struct NewProject{
    pub project_name: String,
    pub project_type: NewType,
    pub current_dir: PathBuf,
}

pub fn get_user_selected() ->anyhow::Result<()> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        active_item_style: Style::new().blue().bold(),
        values_style: Style::new().blue().dim(),
        ..ColorfulTheme::default()
    };
    let project_name = dialoguer::Input::<String>::new()
        .with_prompt("Enter the name of the project")
        .interact()?;
    let current = std::env::current_dir().unwrap();

    match parse_file::check_path(current.join(project_name.clone()).clone()){
        Ok(path) => path,
        Err(e) => {
            return anyhow::bail!("{}",e);
        }
    };
    let selections = &[
        "actix-app",
        "axum-app",
        "ntex-app",
        "dioxus-ssr",
        "dioxus-desktop",
    ];
    let selection = dialoguer::Select::with_theme(&theme)
        .with_prompt("Select the type of project you want to create")
        .items(&selections[..])
        .default(0)
        .interact()
        .unwrap();
    let template_type = match selection {
        0 => NewType::ActixApp,
        1 => NewType::AxumApp,
        2 => NewType::NtexApp,
        3 => NewType::DioxusSsr,
        4 => NewType::DioxusDesktop,
        _ => return anyhow::bail!("Invalid selection"),
    };


    return Ok(create_project(NewProject {
        project_name,
        project_type: template_type,
        current_dir: current,
    })?);
}