use reqwest::{blocking::{Client, ClientBuilder}, redirect::Policy};

fn main() {
    let http_client = Client::new();

    let http_result = http_client.get("https://trevorsullivan.net").send();

    if http_result.is_ok() {
        println!("Body: {:#?}", http_result.ok().unwrap().text().unwrap());
    } else {
        println!("Error occurred: {:#?}", http_result.err());
    }

    let post_result = http_client.post("http://artemis.local:3000/send_data").body("{\"first_name\":\"Trevor\"}").header("User-Agent", "Trevor's Rust Application on Linux").send();
    println!("{:#?}", post_result.ok().unwrap().text());

    // Example: Redirects
    let redir_policy = Policy::limited(5);
    let http_client = ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();
    let http_result = http_client.get("http://artemis.local:3000/weather").send();

    if http_result.is_err() {
        println!("{:#?}", http_result.err());
    }
    
    // println!("Weather app result: {:#?}", http_result);
}
