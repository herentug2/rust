#[tokio::main]
async fn main() {
    let download_route = warp::path("files").and(warp::fs::dir("./files/"));

    let router = download_route.recover(handle_rejection);
    println!("Server started at localhost:8080");
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {

    Ok(warp::reply::with_status(message, code));
}


// in main
let upload_route = warp::path("upload")
    .and(warp::post())
    .and(warp::multipart::form().max_length(5_000_000))
    .and_then(upload);

let router = upload_route.or(download_route).recover(handle_rejection);

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;
