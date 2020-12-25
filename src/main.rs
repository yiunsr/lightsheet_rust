use std::process;
use std::time::Instant;
use datatable::csv_reader;
use web_view::*;

mod datatable;


use actix_rt;
use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::future::Future;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::{borrow::Cow, sync::mpsc, thread};
use web_view::*;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

// fn callback(percent:u32){
//     println!("percent : {}%", percent);
// }

// fn main2() {
//     // let file_path = "D:\\workspace\\vscode\\lightsheet_rust\\res\\euc_kr.csv".to_string();
//     let file_path = "D:\\res\\csv_sample\\1994.csv".to_string();
//     // let file_path = "D:\\res\\csv_sample\\전라북도.txt".to_string();
//     let db_path = "D:\\workspace\\vscode\\lightsheet_rust\\tmp\\tmp.db".to_string();
//     let now = Instant::now();
//     let ret = csv_reader::read_csv(db_path, file_path, callback);
//     if let Err(err) = ret {
//         println!("error running example: {}", err);
//         process::exit(1);
//     }
//     let table_info = ret.unwrap();
//     let spendtime = now.elapsed().as_secs_f64();
//     println!("spendtime : {}", spendtime);
//     println!("1s per insert : {}", (table_info.row_len as f64) / spendtime);

// }


fn assets(req: HttpRequest) -> HttpResponse {
    let path = if req.path() == "/" {
        // if there is no path, return default file
        "index.html"
    } else {
        // trim leading '/'
        &req.path()[1..]
    };

    // query the file from embedded asset with specified path
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn main() {
    let (server_tx, server_rx) = mpsc::channel();
    let (port_tx, port_rx) = mpsc::channel();

    // start actix web server in separate thread
    thread::spawn(move || {
        let sys = actix_rt::System::new("actix-example");
        let server = HttpServer::new(
            || App::new().route("*", web::get().to(assets)))
            .bind("127.0.0.1:0")
            .unwrap();
        
        let port = server.addrs().first().unwrap().port();
        let server = server.run();

        let _ = port_tx.send(port);
        let _ = server_tx.send(server);
        let _ = sys.run();
    });


    let port = port_rx.recv().unwrap();
    let server = server_rx.recv().unwrap();

    // start web view in current thread
    // and point it to a port that was bound
    // to actix web server
    web_view::builder()
        .title("Actix webview example")
        .content(Content::Url(format!("http://127.0.0.1:{}/html/index.html", port)))
        .size(400, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    // gracefully shutdown actix web server
    // let _ = server.stop(true).wait();
    let _ = server.stop(true); //.shutdown_timeout();
}