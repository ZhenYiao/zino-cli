use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::new::args::NewProject;

pub fn create_project(new_project: NewProject) -> std::io::Result<()> {
    let is_actix = new_project.project_type == crate::new::NewType::ActixApp;
    let is_axum = new_project.project_type == crate::new::NewType::AxumApp;
    let is_ntex = new_project.project_type == crate::new::NewType::NtexApp;
    let is_dioxus_ssr = new_project.project_type == crate::new::NewType::DioxusSsr;
    let is_dioxus_desktop = new_project.project_type == crate::new::NewType::DioxusDesktop;
    let is_minimal = new_project.project_type == crate::new::NewType::MinimalApp;
    let is_dioxus_project = is_dioxus_ssr.clone() || is_dioxus_desktop.clone();
    let is_web_backend = is_actix.clone() || is_axum.clone() || is_ntex.clone();

    let path = new_project
        .current_dir
        .join(new_project.project_name.clone());
    {
        copy_binary_file(
            include_bytes!("./template/src/extension/mod.rs"),
            path.join("src/extension/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/domain/mod.rs"),
            path.join("src/domain/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/logic/mod.rs"),
            path.join("src/logic/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/local/docs/rapidoc.html"),
            path.join("local/docs/rapidoc.html"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/service/mod.rs"),
            path.join("src/service/mod.rs"),
        )?;
    }

    if is_web_backend {
        copy_binary_file(
            include_bytes!("./template/src/controller/mod.rs"),
            path.join("src/controller/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/controller/hello.rs"),
            path.join("src/controller/hello.rs"),
        )?;

        copy_binary_file(
            include_bytes!("./template/src/model/mod.rs"),
            path.join("src/model/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/model/tag.rs"),
            path.join("src/model/tag.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/model/user.rs"),
            path.join("src/model/user.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/schedule/mod.rs"),
            path.join("src/schedule/mod.rs"),
        )?;
    } else if is_dioxus_project {
        copy_binary_file(
            include_bytes!("./template/src/view/overview.rs"),
            path.join("src/view/overview.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/src/view/mod.rs"),
            path.join("src/view/mod.rs"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/index.html"),
            path.join("public/index.html"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/favicon.ico"),
            path.join("public/favicon.ico"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/404.html"),
            path.join("public/404.html"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/icons/icon.ico"),
            path.join("public/icons/icon.ico"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/icons/32x32.png"),
            path.join("public/icons/32x32.png"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/icons/64x64.png"),
            path.join("public/icons/64x64.png"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/icons/128x128.png"),
            path.join("public/icons/128x128.png"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/icons/128x128@2x.png"),
            path.join("public/icons/128x128@2x.png"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/css/bulma.min.css"),
            path.join("public/css/bulma.min.css"),
        )?;
        copy_binary_file(
            include_bytes!("./template/public/css/custom.css"),
            path.join("public/css/custom.css"),
        )?;
    }

    let handlebars = Handlebars::new();
    let mut handlebars_dir = Vec::new();
    handlebars_dir.extend_from_slice(&vec![
        ("Cargo.toml", include_str!("./template/Cargo.hbs")),
        ("src/main.rs", include_str!("./template/src/main.hbs")),
        (
            "config/config.dev.toml",
            include_str!("./template/config/config.dev.hbs"),
        ),
        (
            "config/config.prod.toml",
            include_str!("./template/config/config.prod.hbs"),
        ),
        (
            "src/router/mod.rs",
            include_str!("./template/src/router/mod.hbs"),
        ),
    ]);
    if is_web_backend {
        handlebars_dir.extend_from_slice(&vec![
            (
                "src/middleware/access.rs",
                include_str!("./template/src/middleware/access.hbs"),
            ),
            (
                "src/middleware/mod.rs",
                include_str!("./template/src/middleware/mod.hbs"),
            ),
        ]);
    }
    let handlebars_bridle = json!({
        "is_actix": is_actix,
        "is_axum": is_axum,
        "is_ntex": is_ntex,
        "is_dioxus_ssr": is_dioxus_ssr,
        "is_dioxus_desktop": is_dioxus_desktop,
        "is_minimal": is_minimal,
        "is_dioxus_project": is_dioxus_project,
        "is_web_backend": is_web_backend,
        "project_name": new_project.project_name,
    });
    for (i, j) in handlebars_dir {
        render_and_write_to_file(&handlebars, j, &handlebars_bridle, path.join(i))?;
    }

    Ok(())
}
fn copy_binary_file<T: AsRef<Path>>(file_bytes: &[u8], target_path: T) -> std::io::Result<()> {
    if let Some(parent) = target_path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let mut target_file = File::create(target_path)?;
    target_file.write_all(file_bytes)
}

pub(crate) fn check_path(path: PathBuf) -> Result<PathBuf, String> {
    if fs::read_dir(path.clone()).is_err() {
        fs::create_dir(path.clone()).unwrap();
    } else {
        return Err("Directory already exists".to_string());
    }
    Ok(path)
}
fn render_and_write_to_file<T: AsRef<Path>>(
    handlebars: &Handlebars,
    template: &str,
    data: &impl serde::Serialize,
    file_path: T,
) -> std::io::Result<()> {
    let template = template.replace("&lt;", "<").replace("&gt;", ">");
    let rendered = handlebars.render_template(&template, data).unwrap();
    if let Some(parent) = file_path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(file_path)?;
    file.write_all(rendered.as_bytes())?;

    Ok(())
}
