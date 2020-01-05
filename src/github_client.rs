use reqwest::blocking::Response;
use std::cmp::Ordering;

pub trait SortBy {
    fn sort_by<F>(&mut self, compare: F) -> Self
    where
        F: FnMut(&Self, &Self) -> Ordering;

    fn sort_descending_order(&mut self, element: &str) -> Self;
}

impl SortBy for serde_json::Value {
    fn sort_by<F>(&mut self, mut compare: F) -> Self
    where
        F: FnMut(&Self, &Self) -> Ordering,
    {
        self.as_array_mut().unwrap().sort_by(|a, b| compare(a, b));

        self.to_owned()
    }

    fn sort_descending_order(&mut self, element: &str) -> Self {
        self.sort_by(|a, b| {
            if a[element].as_str().is_some() {
                a[element]
                    .as_str()
                    .unwrap()
                    .cmp(&b[element].as_str().unwrap())
            } else if a[element].as_u64().is_some() {
                a[element]
                    .as_u64()
                    .unwrap()
                    .cmp(&b[element].as_u64().unwrap())
            } else {
                // TODO ほかはいいべ…
                unreachable!()
            }
        })
    }
}

#[test]
fn sort_descending_order_test() {
    let data = r#"
        [
            {"number": 2},
            {"number": 3},
            {"number": 1}
        ]"#;

    let mut v: serde_json::Value = serde_json::from_str(data).unwrap();
    let sorted_v = v.sort_descending_order("number");
    assert_eq!(sorted_v[0]["number"].as_u64().unwrap(), 1);
    assert_eq!(sorted_v[1]["number"].as_u64().unwrap(), 2);
    assert_eq!(sorted_v[2]["number"].as_u64().unwrap(), 3);
}

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
