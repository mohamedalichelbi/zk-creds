use axum::{
    routing::{Router, post},
    http::StatusCode,
    Json,
};
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use shared::types::ScriptLang;
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, ResponseFormat, Role},
    Client,
};


pub fn ai_router() -> Router {
    Router::new()
        .route("/genscript", post(gen_script))
        //.route("/checkscript", get(check_script))
}


#[derive(Serialize)]
pub struct GenScriptResponse {
    script: String,
}

#[derive(Deserialize)]
pub struct GenScriptArgs {
    lang: ScriptLang,
    // Description of the credentials' schemas if possible (good)
    // If not, then the crednetials themselves (OpenAI sees them)
    cred_schemes: Vec<String>,
    requirements: String,
}

// generate a script with chatGPT
pub async fn gen_script(Json(payload): Json<GenScriptArgs>) -> (StatusCode, Json<GenScriptResponse>) {
    let ai_client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            ChatCompletionRequestMessage {
                role: Role::System,
                content: Option::Some(format!(
                    "You are a helpful code generation tool for the {:?} language",
                    to_string(&payload.lang))
                ),
                name: Option::None,
                function_call: Option::None
            },
            ChatCompletionRequestMessage {
                role: Role::User,
                content: Option::Some(format!(r#"
                    ```credentials = [{:?}]```.
                    Above are the descriptions of some objects in a variable of type Array called "credentials".
                    Assume the "credentials" variable is defined.
                    Write a script in {:?} that only returns true if the user credentials satisfy the following requirements:
                    {}.
                    Write the shortest and most performant script. Only use variables if necessary, and give them single letter names.
                    "#,
                    payload.cred_schemes.join(","),
                    to_string(&payload.lang),
                    &payload.requirements,
                )),
                name: Option::None,
                function_call: Option::None
            }
        ])
        .build()
        .unwrap();
    let response = ai_client.chat().create(request).await.unwrap();

    (
        StatusCode::OK,
        Json(GenScriptResponse {
            script: response.choices[0].message.content.as_ref().unwrap().to_string(),
        })
    )
}

