pub const SERVER_LIST_PATH: &str = "./data/server-list.dat";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServerListEntry {
    pub ip: String,
    pub port: String,
    pub ping_response: Option<craftping::Response>
}
