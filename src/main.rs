use std::process;
use std::time::Instant;
use web_view::*;
use webbrowser;
use serde_json::{Value, Map};

use actix_rt;
use actix_web::{
    get, body::Body, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use qstring::QString;

use std::{borrow::Cow, sync::mpsc, thread};
use std::sync::mpsc::{{Sender, Receiver}};
use std::sync::Mutex;


use datatable::csv_reader;

mod bridge;
mod datatable;

#[cfg(debug_assertions)]
#[derive(RustEmbed)]
#[folder = "frontend/public/"]
struct Asset;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
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



#[cfg(target_os = "macos")]
fn hide_console_window() {
}
#[cfg(target_os = "windows")]
fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}
#[cfg(target_os = "linux")]
fn hide_console_window() {
}

#[cfg(debug_assertions)]
fn get_port() -> Option<u16> {
    Some(8080u16)
}

#[cfg(not(debug_assertions))]
fn get_port() -> Option<u16> {
    None
}

#[cfg(debug_assertions)]
fn get_bind_url() -> String {
    "127.0.0.1:9010".to_string()
}

#[cfg(not(debug_assertions))]
fn get_bind_url() -> String {
    "127.0.0.1:0".to_string()
}

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


#[get("/simulated_api")]
async fn simulated_api(req: HttpRequest) -> String {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let callback = qs.get("callback").unwrap();
    let agr_param = qs.get("param").unwrap();

    let parsed_json:Value = serde_json::from_str(agr_param).unwrap();
    let parsed_json: Map<String, Value> = parsed_json.as_object().unwrap().clone();
    let api = parsed_json["api"].as_str().unwrap().to_string();
    let cb= parsed_json["cb"].as_str().unwrap().to_string();

    // let param = parsed_json["param"].as_object().unwrap();
    // let wv_opt = None;
    // let api_result = bridge::invoke_handler_internal(wv_opt, &api, cb, param);
    // let ret = format!("{}({})", callback, api_result);
    // ret
    "Test".to_string()
}

fn main() {
    hide_console_window();
    let (server_tx, server_rx) = mpsc::channel();
    let (port_tx, port_rx) = mpsc::channel();

    // start actix web server in separate thread
    thread::spawn(move || {
        let bind_url = get_bind_url();
        let sys = actix_rt::System::new("Light Sheet");
        let server = HttpServer::new(
            || App::new()
            .service(simulated_api)
            .route("*", web::get().to(assets)))
            .bind(bind_url)
            .unwrap();
        
        let port = server.addrs().first().unwrap().port();
        let server = server.run();

        let _ = port_tx.send(port);
        let _ = server_tx.send(server);
        // let _ = api_tx.send("Test".to_string());
        let _ = sys.run();
    });

    let port_opt = get_port();
    let default_port = port_rx.recv().unwrap();
    print!("http://127.0.0.1:{}/index.html", default_port);
    let url = format!("http://127.0.0.1:{}/index.html", default_port);
    let port = port_opt.unwrap_or(default_port);
    let server = server_rx.recv().unwrap();

    // start web view in current thread
    // and point it to a port that was bound
    // to actix web server
    web_view::builder()
        .title("Lightsheet")
        .content(Content::Url(format!("http://127.0.0.1:{}/index.html", port)))
        .size(800, 600)
        // .min_size(600i32, 480i32)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(bridge::invoke_handler)
        .run()
        .unwrap();

    // gracefully shutdown actix web server
    // let _ = server.stop(true).wait();
    let _ = server.stop(true); //.shutdown_timeout();
}

