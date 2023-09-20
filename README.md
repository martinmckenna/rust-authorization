## Simple Setup Guide

1. Install latest version of Docker on your machine
2. Create a file called `.env` in the root of this project and copy over the contents of `.env.example` to it
3. After that run `docker compose up --scale nginx=0` (or if you have access to the `make` command, run `make dc-start-local` instead) in a terminal and monitor the logs to see when the Rust has finished compiling
4. Once you see a message in the logs that reads something like `Running target/debug/rust-auth`, you should be good to go
    * I recommend running `docker logs --tail 10 -f rust-auth` to see the last few lines of the logs

## Endpoints

Upon running the server, you should have access to a few endpoints

`GET http://localhost/profile`:

returns gibberish for now! check back later
