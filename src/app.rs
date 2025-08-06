/// Use axum capabilities.
use axum::routing::get;

/// Create our application by creating our router.
pub fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/chat", get(chat))
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// Use trait
use axum::response::IntoResponse;

/// axum handler for "GET /chat" which shows the program's GPT chat response.
/// This shows how to write a handler that uses a third-party AI service.
pub async fn chat(
    axum::extract::RawQuery(query): 
    axum::extract::RawQuery
) -> axum::response::Response {
    let Some(content) = query 
    else {
        return axum::http::StatusCode::NO_CONTENT.into_response()
    };
    let Ok(harmony_encoding) = openai_harmony::load_harmony_encoding(
        openai_harmony::HarmonyEncodingName::HarmonyGptOss
    )
    else {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };
    let conversation = 
        openai_harmony::chat::Conversation::from_messages([
            openai_harmony::chat::Message::from_role_and_content(
                openai_harmony::chat::Role::User, 
                content
            )
        ]);
    let Ok(tokens)  = harmony_encoding.render_conversation_for_completion(
        &conversation, 
        openai_harmony::chat::Role::Assistant, 
        None
    ) 
    else {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };
    (axum::http::StatusCode::OK, format!("{:?}", tokens)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text = server.get("/chat?sunshine").await.text();
        assert!(response_text.contains("sunshine"), "/chat?sunshine {}", response_text);
    }

}
