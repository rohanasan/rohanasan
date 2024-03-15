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
        // Lets see if there was a get request:
        printf("The get request details: %s\n", req.get_request);
        // Lets handle:
        if (eql(req.path, "/"))
            return send_file(default_html_header, "./html/index.html");
        if (eql(req.path, "/come_here"))
            return send_http_response(default_html_header, "<h1>Hello! you came here!</h1>");
        else return send_404();
    } else if (eql(req.method, "POST")){
          printf("The post request details: %s\n", req.post_request);
          return send_http_response(default_html_header, "<h1>Thanks for submitting a form!</h1>");
    } else {
        return send_404();
    }
}

int main()
{
    printf("Listening at http://localhost:8080\n");
    init(8080);
    set_static_folder("./static/"); // make sure it ends with a /
    serve(handle);
}
