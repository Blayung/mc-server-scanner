mod common;

use base64::Engine;

const SERVER_ADDRESS: &str = "127.0.0.1:8888";

fn mc_text_to_string(mut text: craftping::Chat) -> String {
    for extra in text.extra {
        text.text.push_str(&mc_text_to_string(extra));
    }
    text.text
}

#[actix_web::main]
async fn main() {
    actix_web::HttpServer::new(|| actix_web::App::new().route("/", actix_web::web::get().to(|| async {
        let server_list: Vec<common::ServerListEntry> = bincode::deserialize(&std::fs::read(common::SERVER_LIST_PATH).unwrap()).unwrap();

        let mut server_list_html = String::new();
        for entry in server_list {
            server_list_html.push_str("<tr><th>");
            server_list_html.push_str(&entry.ip);
            server_list_html.push_str("</th><th>");
            server_list_html.push_str(&entry.port);
            server_list_html.push_str("</th><th>");
            match entry.ping_response {
                Err(()) => server_list_html.push_str("ERROR"),
                Ok(ping_response) => {
                    server_list_html.push_str("OK</th><th>");
                    server_list_html.push_str(&html_escape::encode_text(&ping_response.version));
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(&ping_response.protocol.to_string());
                    server_list_html.push_str("</th><th>");
                    if let Some(favicon) = ping_response.favicon {
                        server_list_html.push_str("<img src='data:image/png;base64,");
                        server_list_html.push_str(&base64::prelude::BASE64_STANDARD.encode(favicon));
                        server_list_html.push_str("'>");
                    }
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(&html_escape::encode_text(&mc_text_to_string(ping_response.description)));
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(&ping_response.online_players.to_string());
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(&ping_response.max_players.to_string());
                    server_list_html.push_str("</th><th>");
                    if let Some(sample) = ping_response.sample {
                        if let Some(player) = sample.first() {
                            server_list_html.push_str(&html_escape::encode_text(&player.name));
                        }
                    } else {
                        server_list_html.push_str("?");
                    }
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(match ping_response.enforces_secure_chat {
                        None => "?",
                        Some(enforces_secure_chat) => if enforces_secure_chat {
                            "Enforces"
                        } else {
                            "Doesn't enforce"
                        }
                    });
                    server_list_html.push_str("</th><th>");
                    server_list_html.push_str(match ping_response.previews_chat {
                        None => "?",
                        Some(previews_chat) => if previews_chat {
                            "Previews"
                        } else {
                            "Doesn't preview"
                        }
                    });
                    server_list_html.push_str("</th><th>");
                    if let Some(forge_data) = ping_response.forge_data {
                        let forge_data = format!("{:?}", forge_data);
                        server_list_html.push_str(&html_escape::encode_text(&forge_data));
                    }
                    server_list_html.push_str("</th><th>");
                    if let Some(mod_info) = ping_response.mod_info {
                        let mod_info = format!("{:?}", mod_info);
                        server_list_html.push_str(&html_escape::encode_text(&mod_info));
                    }
                }
            }
            server_list_html.push_str("</th></tr>");
        }

        actix_web::HttpResponse::Ok().body(format!(include_str!("server-list.html"), server_list_html))
    }))).bind(SERVER_ADDRESS).unwrap().run().await.unwrap();
}
