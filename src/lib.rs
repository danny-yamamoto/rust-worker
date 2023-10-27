use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Users {
    #[serde(rename = "user_id")]
    user_id: u32,
    #[serde(rename = "email_address")]
    email_address: String,
    #[serde(rename = "created_at")]
    created_at: u32,
    #[serde(rename = "deleted")]
    deleted: u32,
    #[serde(rename = "settings")]
    settings: String,
}

#[event(fetch, respond_with_errors)]
pub async fn main(request: Request, env: Env, ctx: Context) -> Result<Response> {
    // Route HTTP requests to appropriate handlers based on different URLs
    Router::new()
    .get_async("/:id", |_, ctx| async move {
        let id = match ctx.param("id") {
            Some(value) => value,
            None => {
                eprintln!("Error: id parameter not found");
                "default_id"
            }
        };
        console_log!("id: {:?}", id);
        Response::empty()
    })
    .get_async("/", |_, ctx| async move {
        // To handle asynchronous HTTP GET requests
        // `|_, ctx|`` is a closure
        let d1 = ctx.env.d1("DB")?;
        let statement = d1.prepare("select * from users");
        let res = statement.all().await?;
        Response::from_json(&res.results::<Users>().unwrap())
    }).run(request, env).await
}
