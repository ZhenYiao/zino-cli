use crate::new::parse_file;
use crate::new::parse_file::create_project;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use std::path::PathBuf;
use crate::utils::zino_hello;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum NewType {
    ActixApp,
    AxumApp,
    NtexApp,
    DioxusSsr,
    DioxusDesktop,
    MinimalApp,
}

#[derive(Clone)]
pub struct NewProject {
    pub project_name: String,
    pub project_type: NewType,
    pub current_dir: PathBuf,
}

pub async fn new_parse() -> anyhow::Result<()> {
    zino_hello();
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        active_item_style: Style::new().blue().bold(),
        values_style: Style::new().blue().dim(),
        ..ColorfulTheme::default()
    };
    let project_name = dialoguer::Input::<String>::new()
        .with_prompt(
            ansi_term::Color::Cyan
                .paint("Enter the project name:")
                .to_string(),
        )
        .interact()?;
    let current = std::env::current_dir().unwrap();

    match parse_file::check_path(current.join(project_name.clone()).clone()) {
        Ok(path) => path,
        Err(e) => {
            return anyhow::bail!("{}", e);
        }
    };
    let selections = &[
        ansi_term::Color::RGB(255, 128, 0)
            .paint("Actix App")
            .to_string(),
        ansi_term::Color::RGB(255, 255, 0)
            .paint("Axum App")
            .to_string(),
        ansi_term::Color::RGB(128, 255, 0)
            .paint("Ntex App")
            .to_string(),
        ansi_term::Color::RGB(0, 255, 255)
            .paint("dioxus-ssr")
            .to_string(),
        ansi_term::Color::RGB(127, 0, 255)
            .paint("dioxus-desktop")
            .to_string(),
    ];
    let selection = dialoguer::Select::with_theme(&theme)
        .with_prompt(
            ansi_term::Color::Cyan
                .paint("Select the project template:")
                .to_string(),
        )
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
    }).await?);
}
