// Copyright (c) Rohan Vashisht
// You can read the license inside the LICENSE file

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef _WIN32
    perror("Rohanasan doesn't support windows yet.");
    exit(EXIT_FAILURE);
#else
    // person is using the right os.
#endif

void write_to_file(const char *file_name, const char *content) {
    FILE *file = fopen(file_name, "w");
    if (file != NULL) {
        fputs(content, file);
        fclose(file);
    } else {
        perror("Error opening file");
        exit(EXIT_FAILURE);
    }
}

void create_directory(const char *path) {
    #ifdef _WIN32
    perror("Rohanasan doesn't support windows yet.");
    exit(EXIT_FAILURE);
    #else
    char command[1024];
    snprintf(command, sizeof(command), "mkdir %s", path);
    system(command);
    #endif
}

void download_file(const char *url, const char *file_name) {
    char command[256];
    sprintf(command, "curl -o %s %s", file_name, url);
    system(command);
}

int main() {
    char proj_name[256];
    char compiler_name[256];

    printf("Enter the project name (it would be the directory name): ");
    fgets(proj_name, sizeof(proj_name), stdin);
    proj_name[strcspn(proj_name, "\n")] = '\0'; // Removing trailing newline

    printf("Enter the compiler you would use (gcc/clang/tcc/other): ");
    fgets(compiler_name, sizeof(compiler_name), stdin);
    compiler_name[strcspn(compiler_name, "\n")] = '\0'; // Removing trailing newline

    char make_file_content[1024];
    snprintf(make_file_content, sizeof(make_file_content),
             "PROJNAME = %s\n\n"
             "CC = %s\n\n"
             "CDEBUGFLAGS = -Wall -I./include\n\n"
             "CRELEASEFLAGS = -O3 -I./include\n\n"
             "INCDIR = ./include\n\n"
             "SRC = ./server.c\n\n"
             "TARGET = $(PROJNAME).out\n\n"
             ".PHONY: debug\n"
             "debug:\n"
             "\t@echo \"\\033[0;32mBuilding $(PROJNAME).out for debugging!!!\\033[0m\"\n"
             "\t$(CC) $(CDEBUGFLAGS) $(SRC) -o $(TARGET)\n"
             "\t@echo \"\\033[0;32mBuilt $(PROJNAME).out for debugging!!!\\033[0m\"\n\n"
             ".PHONY: release\n"
             "release:\n"
             "\t@echo \"\\033[0;32mBuilding $(PROJNAME).out for release!!!\\033[0m\"\n"
             "\t$(CC) $(CRELEASEFLAGS) $(SRC) -o $(TARGET)\n"
             "\t@echo \"\\033[0;32mBuilt $(PROJNAME).out for release!!!\\033[0m\"\n\n"
             ".PHONY: run\n"
             "run: $(TARGET)\n"
             "\t./$(TARGET)\n\n"
             ".PHONY: clean\n"
             "clean:\n"
             "\trm -rf $(TARGET)\n\n",
             proj_name, compiler_name);

    create_directory(proj_name);
    chdir(proj_name);
    create_directory("include");
    create_directory("html");
    create_directory("static");
    chdir("include");
    download_file("https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/include/rohanasan.h", "rohanasan.h");
    download_file("https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/include/string_helper.h", "string_helper.h");
    chdir("../");
    write_to_file("makefile", make_file_content);
    write_to_file("server.c", 
"// This is a file that you can edit as per your needs!\n"
"#include \"./include/rohanasan.h\"\n\n"
"const char* myhandle(struct request req)\n"
"{\n"
"    if (eql(req.method, \"GET\")) {\n"
"        if (eql(req.path, \"/\"))\n"
"            return send_file(default_html_header, \"./html/index.html\");\n"
"        if (eql(req.path, \"/come_here\"))\n"
"            return send_http_response(default_html_header, \"<h1>Hello! you came here!</h1>\");\n"
"	else return send_404();\n"
"    } else {\n"
"        return send_404();\n"
"    }\n"
"}\n\n"
"int main()\n"
"{\n"
"    printf(\"Listening at http://localhost:8080\\n\");\n"
"    init(8080);\n"
"    serve(myhandle);\n"
"}\n");

    write_to_file(".gitignore", "*.out");

    chdir("./static");
    download_file("https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/static/rohanasan.png", "rohanasan.png");
    download_file("https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/static/a.txt", "a.txt");
    chdir("../");

    chdir("./html");
    download_file("https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/html/index.html", "index.html");

    printf("Thanks for choosing Rohanasan! Hope you will have a nice coding experience here!\n");
    printf("Your project \"%s\" has been made successfully and you can run it by running:\n", proj_name);
    printf("cd %s\n", proj_name);
    printf("make release run\n");
    return 0;
}
