# Backend API in rust for Beginner

## Goal

This project is the result of udemy rust course called [Back-end web development in Rust for beginners](https://www.udemy.com/course/web-dev-with-rust-rocket-diesel)

## Original Source Code from Course

[Rust Chat](https://gitlab.com/udemy-paris/rocket-app.git)

To run this project, create a file `database.sqlite` at project root level.


## Setup 

```shell
cargo install diesel_cli --no-default-features --features sqlite
```

To test an endpoint:
```shell
curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic Zm9vOmJhcg=='
```