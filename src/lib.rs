// MIT License
//
// Copyright (c) 2024 Rohan Vashisht
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


//! # Rohanasan: An extremely fast backend framework built for rust
//!
//! - Made with Performance, optimization and ease of use in mind.
//!
//! - Currently available in C/C++/Rust programming languages only.
//!
//! - Please use a linux/unix/mac kind of os.
//!
//! - This library has been built from scratch.
//!
//! # How to use in your project?
//! - Open terminal inside the parent folder where you would like to create the folder of your project
//! - Run:
//! ```shell
//! cargo new myproj
//! cd myproj
//! cargo add rohanasan
//! ```
//! - For a start you can add this to main.rs:
//!
//! ```no_run
//! use rohanasan::{
//!     rohanasan, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
//! };
//!
//! fn handle(req: Request) -> String {
//!     if req.path == "/" {
//!         send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
//!     }
//!     else {
//!         send_http_response(DEFAULT_HTML_HEADER, "<h1>404</h1>", req.data)
//!     }
//! }
//!
//! fn main() {
//!     rohanasan! {
//!         serve("0.0.0.0:8080", handle).await;
//!     }
//! }
//!
//! ```
//! - `cargo run` to run your project.
//! - Go to: `localhost:8080`.
//! - Enjoy using Rohanasan!
//! # Performance statistics:
//! # How to run the example?
//! ```shell
//! git clone https://github.com/rohanasan/rohanasan-rs.git
//! cd rohanasan-rs
//! cd examples
//! cargo run --example example
//! ```
//!
//! ## Discord server link:
//! [https://discord.gg/Yg2A3mEret](https://discord.gg/Yg2A3mEret)
//!
//! ### Current Features:
//! - Can run a server at a specified port
//! - Can serve a folder named static at /static
//! - Can send files as http response
//! - Can give you the path, method and protocol
//! ### TODO:
//! - Add feature to change the directory path of the public folder ☑️ Done!!!!
//! - Asynchronous file request handling ☑️ Done!!!!
//! - Add feature to give the user an option to add index.html to static folder ☑️ Done!!!! you can send ./html/static_index.html at /static
//! - Add statistics of performance.
//! - Add feature to... currently it's just a pre alpha release I have to add a lot of features right now!
//!
//! ### Contribute:
//! [https://www.buymeacoffee.com/rohanvashisht](https://www.buymeacoffee.com/rohanvashisht)
//!
//! Please star rohanasan's github repo:
//!
//! [https://github.com/rohanasan/rohanasan-rs](https://github.com/rohanasan/rohanasan-rs)
//!
//! # Examples
//! - **Hello world (Html):**
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{
//!     rohanasan, send_file, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
//! };
//!
//! fn handle(req: Request) -> String {
//!     send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
//! }
//!
//! fn main() {
//!     rohanasan! {
//!         serve("0.0.0.0:8080", handle).await;
//!     }
//! }
//! ```
//! - **Hello world (Html File):**
//! > Basic Html implementation of hello world:
//! ```no_run
//! use rohanasan::{
//!     rohanasan, send_file, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
//! };
//!
//! fn handle(req: Request) -> String {
//!     send_file(DEFAULT_HTML_HEADER, "./html/hello.html", req.data)
//! }
//!
//! fn main() {
//!     rohanasan! {
//!         serve("0.0.0.0:8080", handle).await;
//!     }
//! }
//! ```
//! # Points to remember:
//! - There is no need to import async_std for using rohanasan macro.
//! - By default rohanasan serves any folder named static present in the same directory where you are running the server.


use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
pub use async_std::task::block_on;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use async_std::task;


#[macro_export]
macro_rules! rohanasan {
    // Define the macro pattern.
    ($($body:tt)*) => {
        use $crate::block_on as why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da;
        // Use async-std task spawning to run the asynchronous block provided.
        why_will_someone_use_this_as_a_name_to_import_task_32194ilqrjf8da(async {
            $($body)*
        });
    };
}

pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/html";
pub const DEFAULT_404_HEADER: &str = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html";
pub struct Request {
    pub method: &'static str,
    pub path: &'static str,
    pub get_request: &'static str,
    pub data: bool,
    pub protocol: &'static str,
}

pub async fn handle_connection<F>(mut stream: TcpStream, func: F) -> std::io::Result<()>
where
    F: Fn(Request) -> String + Send,
{
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    if n == 0 {
        return Ok(());
    }

    let request = &buffer[..n];

    // Parse HTTP headers
    let mut headers: Vec<&[u8]> = Vec::new();
    let mut current_header_start = 0;
    for i in 0..n - 1 {
        if request[i] == b'\r' && i + 1 < request.len() && request[i + 1] == b'\n' {
            headers.push(&request[current_header_start..=i]);
            current_header_start = i + 2;
        }
        if request[i] == b'\n' {
            //  The request maker has done some serious mistake in doing so. But, don't worry, I forgive them.
            headers.push(&request[current_header_start..=i]);
            current_header_start = i + 2;
        }
        if request[i] == b'\r'
            && i + 3 < request.len()
            && request[i + 1] == b'\n'
            && request[i + 2] == b'\r'
            && request[i + 3] == b'\n'
        {
            break;
        }
        if request[i] == b'\n' && i + 1 < request.len() && request[i + 1] == b'\n' {
            break;
        }
        // else { "rohanasan received only the header and not any request clubbed to it. And, if it wasn't just the header, along with non-utf8 characters, what are you doing? just think about yourself once.... How did you manage to send such a bad request like that? Please, sit down, relax, enjoy a cup of coffee, and then create a valid request :) " }
    }

    let mut method: &'static str = "POST";
    let mut path: &'static str = "";
    let mut get_request: &'static str = "";
    let mut protocol: &'static str = "";
    let mut keep_alive = false;
    let mut request_was_correct = true;

    for i in headers {
        let line_of_header = String::from_utf8(i.to_vec());
        match line_of_header {
            Ok(line_of_header) => {
                let our_line = line_of_header.trim().to_lowercase();
                if our_line.starts_with("get") {
                    method = "GET";
                    let tokens = our_line
                        .clone()
                        .leak()
                        .split_whitespace()
                        .collect::<Vec<&str>>(); // leaks :cry:, just like how tears leak. XD
                    if tokens.len() > 1 {
                        if tokens[1].contains('?') {
                            let parts: Vec<&str> = tokens[1].split('?').collect();
                            if parts[0].as_bytes()[parts[0].len() - 1] == "/".as_bytes()[0]
                                && parts[0] != "/"
                            {
                                path = &parts[0][..parts[0].len() - 1];
                            } else {
                                path = parts[0];
                            }
                            if parts.len() > 1 {
                                get_request = parts[1];
                            }
                        } else {
                            if tokens[1].as_bytes()[tokens[1].len() - 1] == "/".as_bytes()[0]
                                && tokens[1] != "/"
                            {
                                path = &tokens[1][..tokens[1].len() - 1];
                            } else {
                                path = tokens[1];
                            }
                        }
                    }
                    if tokens.len() > 2 {
                        protocol = tokens[2];
                    }
                }
                if our_line.starts_with("connection")
                    && our_line.len() > 11
                    && our_line.contains("keep-alive")
                {
                    keep_alive = true;
                }
            }
            Err(_) => {
                request_was_correct = false;
            }
        }
    }
    // // Check if the connection is keep-alive or closed
    // let response = func(thing_to_send_to_programmers_function);
    //
    // stream.write_all(response.as_bytes()).await?;
    // stream.flush().await?;
    if request_was_correct {
        if path.starts_with("/static/") && path.len()>8 {
            let file_path = ".".to_owned() + path;
            if Path::new(&file_path).exists(){
                let mut content = Vec::new();
                let mut file = File::open(&file_path)?;
                let _ = file.read_to_end(&mut content);
                let content_type = determine_content_type(&*file_path);
                let mut response_headers = String::new();
                if keep_alive{
                    response_headers = format!(
                        "HTTP/1.1 200 OK\r\nConnection: Keep-Alive\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                        content.len(),
                        content_type
                    );
                }else{
                    response_headers = format!(
                        "HTTP/1.1 200 OK\r\nConnection: Close\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                        content.len(),
                        content_type
                    );
                }
                let mut response = response_headers.into_bytes();
                response.extend_from_slice(&content);
                stream
                    .write_all(&response)
                    .await
                    .expect("Fail to send");
                Ok(())
            }
            else{
                let answer = "HTTP/1.1 404 Not Found\r\nConnection: close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>404</h1>";
                stream
                    .write_all(answer.as_bytes())
                    .await
                    .expect("Fail to send");
                stream.flush().await.expect("Failed to send");
                Ok(())
            }
        }
        else {
            let thing_to_send_to_programmers_function: Request = Request {
                method,
                path,
                get_request: "",
                data: keep_alive,
                protocol,
            };
            let answer = func(thing_to_send_to_programmers_function);

            stream
                .write_all(answer.as_bytes())
                .await
                .expect("Fail to send");
            Ok(())
        }
    } else {
        let answer = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-length: 46\r\nContent-type: text/html\r\n\r\n<h1>An invalid http request was received.</h1>";
        stream
            .write_all(answer.as_bytes())
            .await
            .expect("Fail to send");
        stream.flush().await.expect("Failed to send");
        Ok(())
    }
}

pub async fn serve<F>(address: &str, func: F)
where
    F: Fn(Request) -> String + Send + 'static + Copy,
{
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind to the given address, maybe this port is already in use?");
    println!("Server listening on {}", address);

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream.expect("Failed to create a TCP stream.");
        task::spawn(handle_connection(stream, func));
    }
}

pub fn send_http_response(header: &str, body: &str, keep_alive: bool) -> String {
    if keep_alive {
        format!(
            "{}\r\nContent-Length:{}\nConnection:Keep-Alive\r\n\r\n{}",
            header,
            body.len(),
            body
        )
    } else {
        format!(
            "{}\r\nContent-Length:{}\nConnection:Close\r\n\r\n{}",
            header,
            body.len(),
            body
        )
    }
}

pub fn send_file(header: &str, file_path: &str, keep_alive: bool) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Please place the html files at the correct place, also check the directory from where you are running this server");
    send_http_response(header, &contents, keep_alive)
}

fn determine_content_type(file_path: &str) -> String {
    match file_path.rsplit('.').next() {
        Some("css") => String::from("text/css"),
        Some("txt") => String::from("text/plain"),
        Some("js") => String::from("application/javascript"),
        Some("png") => String::from("image/png"),
        Some("jpg") | Some("jpeg") => String::from("image/jpeg"),
        Some("gif") => String::from("image/gif"),
        Some("pdf") => String::from("application/pdf"),
        Some("htm") | Some("html") => String::from("text/html"),
        _ => String::from("application/octet-stream"),
    }
}