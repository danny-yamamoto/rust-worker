
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
    let d1 = env.d1("DB")?;
    let statement = d1.prepare("SELECT * FROM users");
    let result = statement.all().await?;
    console_log!("result: {:?}", result.results::<Users>().unwrap());

    //Response::empty()
    Response::from_json(&result.results::<Users>().unwrap())
}
