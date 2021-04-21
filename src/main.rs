use http::Uri;
use std::process::Command;
use tide::prelude::*;
use tide::{Body, Error, Request, Response};

#[derive(Deserialize)]
struct Person {
    name: String,
    url: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/person").post(person);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn person(mut req: Request<()>) -> tide::Result {
    let Person { name, url } = req.body_json().await?;

    if name.contains("..") {
        return Err(Error::from_str(
            422,
            "{name} can't contain '..' due to security reasons.",
        ));
    }

    if let Err(_) = &url.parse::<Uri>() {
        return Err(Error::from_str(
            422,
            "{url} must be a proper http / https url.",
        ));
    }

    let input = format!("/tmp/{}.jpg", name);
    let output = format!("/tmp/{}.webp", name);

    Command::new("curl")
        .arg(url.clone())
        .arg("-o")
        .arg(input.clone())
        .output()
        .expect("failed to execute process");

    Command::new("cwebp")
        .arg("-mt")
        .arg("-resize")
        .arg("220")
        .arg("0")
        .arg(input)
        .arg("-o")
        .arg(output.clone())
        .output()
        .expect("failed to execute process");

    Ok(Response::builder(200)
        .body(Body::from_file(output).await?)
        .content_type("image/webp")
        .build())
}
