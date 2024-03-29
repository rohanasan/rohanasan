// MIT License

// Copyright (c) 2024 Rohan Vashisht

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#include "./string_helper.h"
#include <netinet/in.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

int server_fd;
struct sockaddr_in address;
int opt = 1;
int addrlen = sizeof(address);
int new_socket;

struct request {
  const char *path;
  const char *method;
  const char *get_request;
  const char *protocol;
  const char *post_request;
};

typedef const char *(*FuncPtr)(struct request);

extern inline void serve_static_file(int client_socket, char *file_path) {
  FILE *file = fopen(file_path, "rb");
  if (file == NULL) {
    const char *not_found_response = "HTTP/1.1 404 Not Found\nContent-Type: "
                                     "text/html\n\n<h1>404 Not Found</h1>";
    write(client_socket, not_found_response, strlen(not_found_response));
    return;
  }

  // Determine content type based on file extension
  const char *extension = strrchr(file_path, '.');
  const char *content_type = "text/html";
  if (extension && strcmp(extension, ".css") == 0) {
    content_type = "text/css";
  } else if (extension && strcmp(extension, ".js") == 0) {
    content_type = "application/javascript";
  } else if (extension && strcmp(extension, ".png") == 0) {
    content_type = "image/png";
  } else if (extension && strcmp(extension, ".jpg") == 0) {
    content_type = "image/jpeg";
  } else if (extension && strcmp(extension, ".gif") == 0) {
    content_type = "image/gif";
  } else if (extension && strcmp(extension, ".pdf") == 0) {
    content_type = "application/pdf";
  } else if (extension && strcmp(extension, ".htm") == 0) {
    content_type = "text/html";
  } else if (extension && strcmp(extension, ".html") == 0) {
    content_type = "text/html";
  } else {
    // If the file extension is unknown, set content type to octet-stream
    content_type = "application/octet-stream";
  }

  // Send HTTP response header with correct content type
  char response_header[BUFFER_SIZE];
  snprintf(response_header, sizeof(response_header),
           "HTTP/1.1 200 OK\nContent-Type: %s\n\n", content_type);
  write(client_socket, response_header, strlen(response_header));

  // Send file content
  char buffer[BUFFER_SIZE];
  size_t bytes_read;
  while ((bytes_read = fread(buffer, 1, BUFFER_SIZE, file)) > 0) {
    write(client_socket, buffer, bytes_read);
  }

  fclose(file);
}

extern inline void *handle_request(void *arg) {
  char buffer[BUFFER_SIZE];
  FuncPtr the_function_to_send_to_handle = (FuncPtr)arg;

  // Read the HTTP request
  ssize_t bytes_read = read(new_socket, buffer, BUFFER_SIZE);
  if (bytes_read == -1) {
    perror("read");
    close(new_socket);
    pthread_exit(NULL);
  }
  printf("THIS: %s\n", buffer);

  char method[10];
  char cur_path[600];
  char protocol[100];
  const char *get_request = "";
  const char *post_request = "";
  sscanf(buffer, "%s %s %s", method, cur_path, protocol);
  char *actual_path = (char *)cur_path;
  if (cur_path[strlen(cur_path) - 1] == '/' &&
      !eql(cur_path, "/")) { // if end of url is / remove it and path must
                             // obviously not be eql to /
    cur_path[strlen(cur_path) - 1] = '\0';
  }
  if (strchr(cur_path, '?') != NULL) {
    // it contains ?
    char *token = strtok(cur_path, "?");
    if (token[strlen(token) - 1] == '/' &&
        !eql(token, "/")) { // if end of url is / remove it and path must
                            // obviously not be eql to /
      token[strlen(token) - 1] = '\0';
    }
    actual_path = strdup(token);
    token = strtok(NULL, "?");
    get_request = token;
  }
  struct request req;
  req.method = strdup(method);
  req.path = actual_path;
  req.get_request = get_request;
  req.protocol = strdup(protocol);
  if (eql(method, "POST")) {
    char *last_line = NULL;
    char *token = strtok(buffer, "\n");

    while (token != NULL) {
      last_line = token;
      token = strtok(NULL,
                     "\n");
    }
    req.post_request = last_line;
  }
  if (strcmp(method, "GET") == 0 && strncmp(cur_path, "/static/", 8) == 0) {
    char file_path[256];
    snprintf(file_path, sizeof(file_path), "%s%s", STATIC_FOLDER, cur_path + 8);
    serve_static_file(new_socket, file_path);
  } else {
    const char *the_response_that_user_wants_to_send =
        the_function_to_send_to_handle(req);
    write(new_socket, the_response_that_user_wants_to_send,
          strlen(the_response_that_user_wants_to_send));
  }
  close(new_socket);
  pthread_exit(NULL);
}

extern inline void init(int PORT) {

  // Creating socket file descriptor
  if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
    perror("socket failed");
    exit(EXIT_FAILURE);
  }

  // Forcefully attaching socket to the port 8080
  if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt))) {
    perror("setsockopt");
    exit(EXIT_FAILURE);
  }
  address.sin_family = AF_INET;
  address.sin_addr.s_addr = INADDR_ANY;
  address.sin_port = htons(PORT);
  return;
}

extern inline int serve(FuncPtr handler) {
  // Forcefully attaching socket to the port 8080
  if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
    perror("bind failed");
    exit(EXIT_FAILURE);
  }
  if (listen(server_fd, 1) < 0) {
    perror("listen");
    exit(EXIT_FAILURE);
  }

  pthread_t tid;

  while (1) {
    if ((new_socket = accept(server_fd, (struct sockaddr *)&address,
                             (socklen_t *)&addrlen)) < 0) {
      perror("accept");
      continue;
    }

    // Create a new thread to handle the request
    if (pthread_create(&tid, NULL, handle_request, (void *)handler) != 0) {
      perror("pthread_create");
      close(new_socket);
    }

    // Detach the thread to avoid memory leaks
    pthread_detach(tid);
  }
}
