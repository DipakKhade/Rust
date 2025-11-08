use poem::{
    Body, IntoResponse, Route, Server, error::ReadBodyError, get, handler, http::StatusCode,
    listener::TcpListener, post, web::{Multipart, websocket::WebSocket},
};
use std::fs::File;
use std::io::Write;

#[handler]
async fn process_files(body: Body) -> String {
    let mut res = "".to_string();
    match body.into_string().await {
        Ok(T) => res = T.to_string(),
        Err(_) => res = "error occured".to_string(),
    }
    format!("result: {}", res)
}


#[handler]
async fn upload(mut multipart: Multipart) -> poem::Result<String> {
    while let Some(field) = multipart.next_field().await? {
        let content_type = field
            .content_type()
            .map(|v| v.to_string())
            .unwrap_or_default();
        let file_name = field
            .file_name()
            .map(|v| v.to_string())
            .unwrap_or("file.xlsx".into());

        if content_type != "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" {
            return Err(poem::Error::from_string("Only .xlsx files allowed", StatusCode::BAD_REQUEST));
        }

        let data = field.bytes().await?;
        let mut file = File::create(format!("./uploads/{}", file_name)).unwrap();
        file.write_all(&data).unwrap();

        return Ok(format!("Uploaded: {}", file_name));
    }

    Err(poem::Error::from_string(
        "No file found",
        StatusCode::BAD_REQUEST,
    ))
}


#[handler]
async fn init_ws(ws: WebSocket) -> impl IntoResponse {
    ws.protocols(vec!["graphql-rs", "graphql-transport-ws"])
        .on_upgrade(|socket| async move {

            println!("upgraded to ws connection")

            // socket.
        })
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
                .at("/process/process_file", post(upload))
                .at("/init_ws", get(init_ws));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
