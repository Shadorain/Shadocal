use anyhow::Result;
use google_calendar::Client;
use std::net::TcpListener;

use directories::ProjectDirs;

use url::Url;

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub async fn get_client() -> Result<Client> {
    let data_dir = data_directory()?;
    dotenv::from_filename(data_dir.join("client")).ok();

    if let Some(rtoken) = check_rtoken(&data_dir.join(".rtoken")) {
        let client = Client::new_from_env("", rtoken).await;
        client.refresh_access_token().await?;
        return Ok(client);
    }

    let mut client = Client::new_from_env("", "").await;
    println!(
        "🔗 Open this URL in your browser:\n{}\n",
        client.user_consent_url(&[
            "https://www.googleapis.com/auth/calendar".to_string(),
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ])
    );

    let (code, state) = get_code_state();

    let access_token = client.get_access_token(&code, &state).await?;
    std::fs::write(data_dir.join(".rtoken"), access_token.refresh_token)
        .expect("Failed to write refresh token to file.");

    Ok(client)
}

fn get_code_state() -> (String, String) {
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // The server will terminate itself after collecting the first code.
    let Some(mut stream) = listener.incoming().flatten().next() else {
        panic!("listener terminated without accepting a connection");
    };

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, code)| code.into_owned())
        .unwrap();

    let state = url
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, state)| state.into_owned())
        .unwrap();

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();

    (code, state)
}

pub fn data_directory() -> Result<PathBuf> {
    Ok(
        if let Some(proj_dirs) = ProjectDirs::from("com", "shadorain", "gcal") {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            return Err(anyhow::anyhow!(
                "Unable to find data directory for ShadoGCal"
            ));
        },
    )
}

fn check_rtoken(token_path: &Path) -> Option<String> {
    std::fs::read_to_string(token_path).ok()
}
