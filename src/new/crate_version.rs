use std::fmt::Display;
use rust_i18n::t;
// use loading::{Loading, Spinner};

#[derive(Clone, Debug)]
pub struct Version{
    pub zino: String,
    pub zino_core: String,
    pub zino_dioxus: String,
    pub zino_derive: String,
    pub zino_model: String,
    pub dioxus: String,
    pub dioxus_router: String,
    pub tracing: String,
    pub dioxus_free_icons: String,
    pub actix_web: String,
    pub axum: String,
    pub ntex: String,
    pub serde: String,
}
pub enum Crates{
    Zino,
    ZinoCore,
    ZinoDioxus,
    ZinoDerive,
    ZinoModel,
    Dioxus,
    DioxusRouter,
    Tracing,
    DioxusFreeIcons,
    ActixWeb,
    Axum,
    Ntex,
    Serde,
}
pub static CRATES: [&str; 13] = [
    "zino",
    "zino_core",
    "zino_dioxus",
    "zino_derive",
    "zino_model",
    "dioxus",
    "dioxus_router",
    "tracing",
    "dioxus_free_icons",
    "actix-web",
    "axum",
    "ntex",
    "serde",
];
impl Default for Version {
    fn default() -> Self {
        Self{
            zino: "0.23.3".to_string(),
            zino_core: "0.24.3".to_string(),
            zino_dioxus: "0.7.0".to_string(),
            zino_derive: "0.21.3".to_string(),
            zino_model: "0.21.3".to_string(),
            dioxus: "0.5.6".to_string(),
            dioxus_router: "0.5.6".to_string(),
            tracing: "0.1.40".to_string(),
            dioxus_free_icons: "0.8.6".to_string(),
            actix_web: "4.8.0".to_string(),
            axum: "0.7.5".to_string(),
            ntex: "2.0.3".to_string(),
            serde: "1.0.204".to_string(),
        }
    }
}

impl Version {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn online(&mut self) -> anyhow::Result<()> {
        // let loading = Loading::with_stdout(Spinner::new(vec![""]));
        for (idx,crate_name) in CRATES.iter().enumerate().map(|(x,y)|{(x,y)}){
            let path = format!(" https://crates.io/api/v1/crates/{}", crate_name);
            let client = reqwest::Client::new();
            let resp = client
                .get(&path)
                .header("User-Agent", "reqwest")
                .send()
                .await?;
            let json = resp.json::<serde_json::Value>().await?;
            let version = json.get("crate").unwrap().get("max_stable_version").unwrap().as_str().unwrap();
            match Crates::from(*crate_name){
                Crates::Zino => self.zino = version.to_string(),
                Crates::ZinoCore => self.zino_core = version.to_string(),
                Crates::ZinoDioxus => self.zino_dioxus = version.to_string(),
                Crates::ZinoDerive => self.zino_derive = version.to_string(),
                Crates::ZinoModel => self.zino_model = version.to_string(),
                Crates::Dioxus => self.dioxus = version.to_string(),
                Crates::DioxusRouter => self.dioxus_router = version.to_string(),
                Crates::Tracing => self.tracing = version.to_string(),
                Crates::DioxusFreeIcons => self.dioxus_free_icons = version.to_string(),
                Crates::ActixWeb => self.actix_web = version.to_string(),
                Crates::Axum => self.axum = version.to_string(),
                Crates::Ntex => self.ntex = version.to_string(),
                Crates::Serde => self.serde = version.to_string(),
            }
            println!("{}",&format!("\x1B[32m✔\x1B[0m {} {}/{}",t!("Fetching crate") ,idx + 1,CRATES.len()));

        }
    Ok(())
    }
}

impl Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Crates::Zino => "zino".to_string(),
            Crates::ZinoCore => "zino_core".to_string(),
            Crates::ZinoDioxus => "zino_dioxus".to_string(),
            Crates::ZinoDerive => "zino_derive".to_string(),
            Crates::ZinoModel => "zino_model".to_string(),
            Crates::Dioxus => "dioxus".to_string(),
            Crates::DioxusRouter => "dioxus_router".to_string(),
            Crates::Tracing => "tracing".to_string(),
            Crates::DioxusFreeIcons => "dioxus_free_icons".to_string(),
            Crates::ActixWeb => "actix_web".to_string(),
            Crates::Axum => "axum".to_string(),
            Crates::Ntex => "ntex".to_string(),
            Crates::Serde => "serde".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for Crates {
    fn from(value: &str) -> Self {
        match value {
            "zino" => Crates::Zino,
            "zino_core" => Crates::ZinoCore,
            "zino_dioxus" => Crates::ZinoDioxus,
            "zino_derive" => Crates::ZinoDerive,
            "zino_model" => Crates::ZinoModel,
            "dioxus" => Crates::Dioxus,
            "dioxus_router" => Crates::DioxusRouter,
            "tracing" => Crates::Tracing,
            "dioxus_free_icons" => Crates::DioxusFreeIcons,
            "actix_web" => Crates::ActixWeb,
            "axum" => Crates::Axum,
            "ntex" => Crates::Ntex,
            "serde" => Crates::Serde,
            _ => Crates::Zino,
        }
    }
}

#[tokio::test]
async fn test_crates_version(){
    tracing_subscriber::fmt().init();
    let mut v = Version::new();
    v.online().await.unwrap();
    dbg!(v);
}
