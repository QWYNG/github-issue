use reqwest::blocking::Response;

pub fn fetch(user: &String, project: &String) -> Result<Response, reqwest::Error> {
    const USER_AGENT: &str = "QWYNG/github-issue";

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();

    let github_issue_url = format!("https://api.github.com/repos/{}/{}/issues", user, project);

    client.get(&github_issue_url).send()
}

pub fn handle_response(responce: Response) -> Result<serde_json::Value, String> {
    match responce.status() {
        reqwest::StatusCode::OK => {
            let text = responce.text().unwrap();
            let v: serde_json::Value = serde_json::from_str(&text).unwrap();
            Ok(v)
        }
        status => Err(format!("{}", status)),
    }
}
