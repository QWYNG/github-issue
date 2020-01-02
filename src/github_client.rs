pub fn fetch(user: &String, project: &String) -> Result<(), reqwest::Error> {
    let github_issue_url = format!("https://api.github.com/repos/{}/{}/issues", user, project);
    let resp = get_github_issue(&github_issue_url)?;

    println!("{:?}", resp.text());
    Ok(())
}

fn get_github_issue(
    github_issue_url: &String,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    const USER_AGENT: &str = "QWYNG/github-issue";
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;

    client.get(github_issue_url).send()
}
