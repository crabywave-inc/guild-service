use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub auth_service_url: String,

    #[clap(env)]
    pub google_project_id: String,

    #[clap(env)]
    pub firebase_database: String,

    #[clap(env)]
    pub firebase_project_id: String,
}
