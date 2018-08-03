# A monorepo architecture compatible with Docker #

This repository showcases a method to manage a **monorepo** of several Dockerized services. It enables cleanly sharing code between them.

My goals are:

- Ability to work on both the microservices and the shared libraries locally without docker.
- Ability to build the Docker images very simply and reliably while managing the dependencies.
- Make it language agnostic.

The goals are fulfilled for Node.js and Rust libraries and service atm.

## How to build and run the Docker images ##

To build a single image, run the following.

```console
$ make path_to/service
```

This builds the target image and its dependencies, incremental builds rely on docker layer caches.

for example

```console
$ make services/my-rust-service
```

To build all the images simply run

```console
$ make all
```

To run the Node.js service

```console
$ docker run monorepo/my-node-service
```

To run the Rust service

```console
$ docker run monorepo/my-rust-service
```

in both cases any changes to the underlying libraries (in `bases`) will be taken into account properly in the next build and run.

## How to build and run the Node.js service without Docker ##

```console
$ cd ./services/my-node-service/
$ npm install
$ npm start
```

Thanks to the way `npm install` deals with local dependencies you don't even need to rerun it when a change is done in the shared library (e.g. in `/bases/cool-node/mycoollibrary/src/cool.js`).

## How to build and run the Rust service without Docker ##

```console
$ cd ./services/my-rust-service/
$ cargo run
```

Thanks to the way `cargo run` works, changes to the shared library are detected, there's nothing additional to do.
