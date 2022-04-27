# ![RealWorld Example App](logo.png)

> ### Rust/Axum/SeaORM codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

[Demo](https://github.com/gothinkster/realworld)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

# Overview

This codebase was created to demonstrate a fully fledged backend application built with **Rust** and [**`axum`**](https://github.com/wolf4ood/realworld-axum) including CRUD operations, authentication, routing, pagination, and more.


This project is a fork of [**`RealWorld Tide`**](https://github.com/colinbankier/realworld-tide) with:

- module `web` rewritten with [`Axum`](https://github.com/tokio-rs/axum)
- module `db` rewritten with [`SeaORM`](https://www.sea-ql.org/SeaORM/docs/index)


This project attempts to achieve a clear separation between web, domain and persistence logic -
loosely along the lines of the [ports and adapters architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)).  
These three concerns live in separate crates - `web`, `domain` and `db`.  
`axum` is used in the `web` crate while `diesel` is the main character of the `db` crate.  
The fourth crate, `application`, glues the other three together and provides the runnable binary.

Each sub-crate has its own set of tests, with integration tests taking place in the `web` crate.

You can also exercise the application using Realworld's Postman collection: [here](https://github.com/gothinkster/realworld/tree/master/api).

For more information on how this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

## Other frameworks

If you want to get a taste of what it feels like to build a Realworld's implementation using another
Rust's web framework, you can reuse the `domain` and `db` sub-crate. 

You just need to provide an alternative implementation of the `web` crate leveraging your framework of choice.

# Getting started

## Setup

### Prerequisites

- Rust 1.56 (see [here](https://www.rust-lang.org/tools/install) for instructions)
- Docker/DockerCompose (see [here](https://docs.docker.com/install/) for instructions)
- Make (see [here](https://www.gnu.org/software/make/) for instructions)

### Setup steps
- Launch a local Postgres instance with docker-compose 
```bash
docker-compose up -d
```
- Run SQL migrations

``` bash
make db
```

You are ready to go!

## Run tests
Run tests, including DB integration tests


```bash
make test
```

## Run app and realworld test suite
Run the app
```bash
make run
```


If you want to run the "realworld" Postman tests, just execute
```bash
git clone https://github.com/gothinkster/realworld
cd realworld/api
APIURL=http://localhost:5000/api ./run-api-tests.sh
```

## Configuration

All configuration files are in the `configuration` folder.

The default settings are stored in `configuration/base.yml`.

Environment-specific configuration files can be used to override or supply additional values on top the
default settings (see `development.yml` or `test.yml`).
In a production environment, you could introduce a `production.yml` to store production-specific configuration values.

Configuration files can also be overriden using environment variables, prefixed with `APP`: 
the value of `APP_APPLICATION_PORT` will have higher priority then `application.port` in `base.yml` or `development.yml`.

All configurable parameters are listed in `configuration.rs`.
