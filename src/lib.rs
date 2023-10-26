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
    Router::new().get_async("/", |_, ctx| async move {
        let d1 = ctx.env.d1("DB")?;
        let statement = d1.prepare("select * from users");
        let res = statement.all().await?;
        Response::from_json(&res.results::<Users>().unwrap())
    }).run(request, env).await
}
