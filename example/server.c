// This is an example file which you can run
// by following the steps mentioned here:
// 1) git clone this repo
// 2) cd into rohanasan_c folder
// 3) cd into example directory
// 4) run: gcc -o server ./server.c
// 5) run: ./server
// 6) visit the url: http://localhost:8080
// 7) Enjoy using rohanasan!!!!!

#include "../include/rohanasan.h"

const char* handle(struct request req)
{
    if (eql(req.method, "GET")) {
        if (eql(req.path, "/"))
            return send_file(default_html_header, "./html/index.html");
        if (eql(req.path, "/come_here"))
            return send_http_response(default_html_header, "<h1>Hello! you came here!</h1>");
        else return send_404();
    } else {
        return send_404();
    }
}

int main()
{
    printf("Listening at http://localhost:8080\n");
    init(8080);
    serve(handle);
}
