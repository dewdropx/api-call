use serde::Deserialize; // deserialize json
use reqwest::Error; // reqwest error type return
use reqwest::header::USER_AGENT; // USER_AGENT http header constant

// define user struct
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u64,
}

// set up tokio async main function
#[tokio::main]
async fn main() -> Result<(), Error> {

    // request url
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    println!("{}", request_url);

    // set up client and send get request
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    // check status and handle error
    let status = response.status();
    println!("status: {}", status);

    if !status.is_success() {
        let body = response.text().await?;
        eprintln!("github error body: {}", body);

        return Ok(());
    }
    
    // deserialize json response to vector of users
    let users: Vec<User> = response.json().await?;
    
    for user in &users {
        println!("login: {}, id: {}", user.login, user.id);
    }

    Ok(())
}