# Introducing Rohanasan
## An extremely fast backend framework for multiple programming languages!

> Made with Performance, optimization and ease of use in mind.
> Currently available in C programming language only
> Please use a linux/unix/mac kind of os

# How to run example:
- Run: `git clone https://github.com/rohanasan/rohanasan_c.git`
- Next, Run: `cd rohanasan_c/example`
- Make sure you are using a linux/unix/mac like operating system with gcc/clang installed
- Run: `gcc -o server ./server.c`
  or `clang -o server ./server.c` depending on the compiler you want to use/have installed.
- Run: `./server`
- Visit: [localhost:8080](http://localhost:8080)
- Then: 👏 for yourself, you just ran the rohanasan's example!
- You can host the example folder as well!!! Use the docker file,
  It has all the correct configuration!

# How to use in your project?
- Open terminal inside the parent folder where you would like to create the folder of your project
- Run:
```sh
curl https://raw.githubusercontent.com/rohanasan/rohanasan_c/main/rohanasan_maker.py -o rohanasan_maker.py
python3 ./rohanasan_maker.py
rm ./rohanasan_maker.py
```
- Now, you `cd` into the folder created my the rohanasan project maker software.
- then run:
- `make release run` to run your project!
- Enjoy using Rohanasan!

## Basic hello world looks like this:
```c
#include "./include/rohanasan.h"

const char* myhandle(struct request req)
{
    return send_http_response(default_html_header,
        "<h1>Hello from Rohanasan!</h1>");
}

int main()
{
    init(8080);
    serve(myhandle);
    return 0;
}
```

### Current Features:
- Can run a server at a specified port
- Can serve a folder named static at /static
- Can send files as http responce
- Can give you the path, method and protocol
### TODO:
- Add feature to give the `get` path.
- Add feature to give the `post` path.
- Add feature to change the directory path of the public folder
- Add feature to give the user an option to add index.html to static folder
- Add feature to... currectly its just a pre alpha release I have to add alot of features right now!

### Contribute:
https://www.buymeacoffee.com/rohanvashisht
