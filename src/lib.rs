use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::fs;

const DEBUG: bool = false;

pub const DEFAULT_HTML_HEADER: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/html";
pub const DEFAULT_404_HEADER: &str = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html";
pub struct Request {
    pub method: &'static str,
    pub path: &'static str,
    pub get_request: &'static str,
    pub data: bool,
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

    let mut request = &buffer[..n];

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
                    let mut tokens = our_line.clone().leak().split_whitespace().collect::<Vec<&str>>(); // leaks :cry:, just like how tears leak. XD
                    if tokens.len() > 1 {
                        path = &*tokens[1];
                    }
                    if tokens.len() > 2 {
                        protocol = &*tokens[2];
                    }
                }
                if our_line.starts_with("connection") {
                    if our_line.len() > 11 && our_line.contains("keep-alive") {
                        keep_alive = true;
                    }
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
        let thing_to_send_to_programmers_function: Request = Request {
            method,
            path,
            get_request: "",
            data: keep_alive,
        };
        let answer = func(thing_to_send_to_programmers_function);

        stream
            .write_all(answer.as_bytes())
            .await
            .expect("Fail to send");
        Ok(())
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
    if keep_alive == true {
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
    send_http_response(header, &*contents, keep_alive)
}
