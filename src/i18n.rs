use std::process::Command;
use rust_i18n::set_locale;

#[cfg(target_os = "windows")]
pub fn init_i18n() -> anyhow::Result<()>{
    let args = Command::new("REG")
        .args(&["QUERY", "HKCU\\Control Panel\\International", "/v", "LocaleName"])
        .output()?;
    let output = String::from_utf8(args.stdout)?;
    if output.contains("zh-CN") {
        set_locale("zh-CN");
        tracing::info!("Set locale to zh-CN");
    } else {
        set_locale("en");
        tracing::info!("Set locale to en-US");
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn init_i18n() -> anyhow::Result<()>{
    let args = Command::new("locale")
        .output()?;
    let output = String::from_utf8(args.stdout)?;
    if output.contains("zh_CN") {
        set_locale("zh-CN");
        tracing::info!("Set locale to zh-CN");
    } else {
        set_locale("en");
        tracing::info!("Set locale to en-US");
    }
    Ok(())
}

// #[cfg(target_os = "macos")]
// pub fn init_i18n() -> anyhow::Result<()>{
//     let args = Command::new("defaults read -g AppleLanguages")
//         .output()?;
//     let output = String::from_utf8(args.stdout)?;
//     if output.contains("zh-Hans") {
//         set_locale("zh-CN");
//         tracing::info!("Set locale to zh-CN");
//     } else {
//         set_locale("en");
//         tracing::info!("Set locale to en-US");
//     }
//     Ok(())
// }

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn init_i18n() -> anyhow::Result<()>{
    set_locale("en");
    tracing::info!("Set locale to en-US");
    Ok(())
}

#[test]
fn test_init_i18n(){
    tracing_subscriber::fmt().init();
    init_i18n().ok();
}