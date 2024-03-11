# Copyright (c) Rohan Vashisht
# You can read the license inside the LICENSE file
import os
import subprocess
def write(file_name, content):
    x = open(file_name, "w")
    x.write(content)
    x.close()
def main():
    proj_name = input("Enter the project name (it would be the directory name): ")
    compiler_name = input("Enter the compiler you would use (gcc/clang/tcc/other): ")
    make_file_content = f"""PROJNAME = {proj_name}

CC = {compiler_name}

CDEBUGFLAGS = -Wall -I./include

CRELEASEFLAGS = -O3 -I./include

INCDIR = ./include

SRC = ./server.c

TARGET = $(PROJNAME).out

.PHONY: debug
debug:
	@echo "\\033[0;32mBuilding $(PROJNAME).out for debugging!!!\\033[0m"
	$(CC) $(CDEBUGFLAGS) $(SRC) -o $(TARGET)
	@echo "\\033[0;32mBuilt $(PROJNAME).out for debugging!!!\\033[0m"

.PHONY: release
release:
	@echo "\\033[0;32mBuilding $(PROJNAME).out for release!!!\\033[0m"
	$(CC) $(CRELEASEFLAGS) $(SRC) -o $(TARGET)
	@echo "\\033[0;32mBuilt $(PROJNAME).out for release!!!\\033[0m"

.PHONY: run
run: $(TARGET)
	./$(TARGET)


.PHONY: clean
clean:
	rm -rf $(TARGET)

    """
    os.mkdir("./"+proj_name)
    os.chdir("./"+proj_name)
    os.mkdir("include")
    os.mkdir("html")
    os.mkdir("static")
    os.chdir("include")
    subprocess.run(["curl", "-o", "rohanasan.h", "https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/include/rohanasan.h"])
    subprocess.run(["curl", "-o", "string_helper.h", "https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/include/string_helper.h"])
    os.chdir("../")
    write("./makefile", make_file_content)
    write("./server.c", """// This is a file that you can edit as per your needs!
#include "./include/rohanasan.h"

const char* myhandle(struct request req)
{
    if (eql(req.method, "GET")) {
        if (eql(req.path, "/"))
            return send_file(default_html_header, "./html/index.html");
        if (eql(req.path, "/come_here"))
            return send_http_response(default_html_header, "<h1>Hello! you came here!</h1>");
    } else {
        return send_404();
    }
}

int main()
{
    printf("Listening at http://localhost:8080\\n");
    init(8080);
    serve(myhandle);
}
""")
    write("./.gitignore", "*.out")
    os.chdir("./static")
    subprocess.run(["curl", "-o", "rohanasan.png", "https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/static/rohanasan.png"])
    subprocess.run(["curl", "-o", "a.txt", "https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/static/a.txt"])
    os.chdir("../")
    os.chdir("./html")
    subprocess.run(["curl", "-o", "index.html", "https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/example/html/index.html"])
    print("Thansk for choosing Rohanasan! Hope you will have a nice coding experience here!")
    print(f'You project "{proj_name}" has been made successfully and you can run it by running:')
    print(f'cd {proj_name}')
    print('make release run')
if __name__ == "__main__":
    main()
