# UDP-Client-Server

A multithreaded client and server application written in Rust with a
mocked file system.

## Server

The server has a single consumer thread, that will process all messages that will
be sent to a client. And N producer threads, with N being the number of request being
processed at the server at any moment.

The server has two modes:

1. The first one is when the message starts with: "file: ", when in this mode the server will try to find any the file
   request at the mocked file system.
    * Ex.: file: /photos/photo1.png
2. The second mode is when the message starts with anything besides the "file: ". when in this mode
   the server will answer with the message sent but all in uppercase and with this text appended:
   " (hehe to upper 🤖)".

### The mocked file system

The server has this mocked file system implemented inside:

```
root
├── photos
│   └── photo1.png
├── important.txt
└── ping.txt
```

## Client

The client can send UDP messages to the server.

# About

This project was developed for a subject at University. With the following specifications:

1. Make a server that inverts the string received or change in some way or form.

2. Make it multithreaded.

3. Make the program simulate an file server, where you can send a name and will return the content file

