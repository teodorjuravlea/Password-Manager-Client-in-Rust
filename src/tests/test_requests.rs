use crate::requests;

pub fn test_requests() {
    let reqwest_client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = requests::register_request(
        "lmao@example.com",
        "password",
        "password",
        &reqwest_client,
        "http://localhost:8080/register",
    );

    println!("{}", response.unwrap());

    let response = requests::login_request(
        "lmao@example.com",
        "password",
        &reqwest_client,
        "http://localhost:8080/login",
    );

    println!("{}", response.unwrap());

    let response = reqwest_client.get("http://localhost:8080/me").send();

    println!("{}", response.unwrap().text().unwrap());

    let response = reqwest_client.get("http://localhost:8080/logout").send();

    println!("{}", response.unwrap().text().unwrap());

    let response = reqwest_client.get("http://localhost:8080/me").send();

    println!("{}", response.unwrap().text().unwrap());
}
