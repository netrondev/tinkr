#[server]
async fn sync_data() -> Result<(), ServerFnError> {
    println!("Data synchronization started...");

    let token = std::env::var("COOLIFY_API_TOKEN")?;
    let url = std::env::var("COOLIFY_API_URL")?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/v1/user", url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Response: {:?}", response);

    if response.status().is_success() {
        println!("Token is valid!");
    } else {
        eprintln!("Token validation failed: {}", response.status());
    }

    Ok(())
}
