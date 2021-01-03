use std::process;
use std::time::Instant;
use web_view::*;
use webbrowser;

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

static mut API_REQ_TX: Option<Mutex<Sender<String>>> = None;
static mut API_REQ_RX: Option<Receiver<String>> = None;

static mut API_RES_TX: Option<Mutex<Sender<String>>> = None;
static mut API_RES_RX: Option<Receiver<String>> = None;

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
    let param = qs.get("param").unwrap();
    let mut api_req_tx:Sender<String>;
    let mut api_res_rx:&Receiver<String>;
    unsafe {
        api_req_tx = API_REQ_TX.as_ref().unwrap().lock().unwrap().clone();
        api_res_rx = API_RES_RX.as_ref().unwrap().clone();
    }
    api_req_tx.send(param.to_string());
    println!("Wait");
    api_res_rx.recv().unwrap();

    let api_result = "{\"data\": \"Echo Test\"}";
    let ret = format!("{}({})", callback, api_result);
    ret
}

fn main() {
    hide_console_window();
    let (server_tx, server_rx) = mpsc::channel();
    let (port_tx, port_rx) = mpsc::channel();
    // let (api_tx, api_rx):(Receiver<String>, Sender<String>) = mpsc::channel();

    // https://users.rust-lang.org/t/global-sync-mpsc-channel-is-possible/14476
    unsafe {
        let (api_req_tx, api_req_rx) = mpsc::channel();
        API_REQ_TX = Some(Mutex::new(api_req_tx));
        let api_req_tx = API_REQ_TX.as_ref().unwrap().lock().unwrap().clone();
        API_REQ_RX = Some(api_req_rx);
        let api_req_rx = API_REQ_RX.as_ref().unwrap().clone();

        let (api_res_tx, api_res_rx) = mpsc::channel();
        API_RES_TX = Some(Mutex::new(api_res_tx));
        let api_req_tx = API_RES_TX.as_ref().unwrap().lock().unwrap().clone();
        API_RES_RX = Some(api_res_rx);
        let api_res_rx = API_RES_RX.as_ref().unwrap().clone();
    }

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

