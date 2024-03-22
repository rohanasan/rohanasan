use rohanasan::{
    rohanasan, send_file, send_http_response, serve, Request, DEFAULT_HTML_HEADER,
};

fn handle(req: Request) -> String {
    if req.path == "/" {
        send_http_response(DEFAULT_HTML_HEADER, "<h1>Hello!</h1>", req.data)
    }
    else if req.path == "/hello" {
        send_file(DEFAULT_HTML_HEADER, "./html/hello.html", req.data)
    }
    else {
        send_http_response(DEFAULT_HTML_HEADER, "<h1>404</h1>", req.data)
    }
}

fn main() {
    rohanasan! {
        serve("0.0.0.0:8080", handle).await;
    }
}
