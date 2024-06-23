# Kubernetes project : ara-monitoring

## Prerequisites

- Kubernetes cluster (k3d/k3s, rke2, ...)
- A storage class (local-path, longhorn, ...)
- A secret containing the informations to access to our registry. You can generate one with the command below :

```bash
kubectl create secret docker-registry regcred -n ara-back \
  --docker-server=<registry> \
  --docker-username=<username> \
  --docker-password=<password> \
  --docker-email=<email>
```

- A WASM operator on kubernetes ([Spinkube](https://github.com/spinkube/spin-operator) for example)

PS : We can also install ArgoCD to manage the deployment of the project more easily like I did. You can find the installation steps [here](https://argoproj.github.io/argo-cd/getting_started/).

## WASI API

The goal of this part of the project was to reproduce our API in a WebAssembly virtual machine. Because it seems to be a well-suited language for WASM, I chose Rust.

### Routes

Here is a list of the accessible routes :

    GET / : Return the pid of the database
    GET /users : Return all users in the database
    POST /adduser : Create a user in the database

### Variables

The most important variable is the `db_url` located in the `api-wasi/spin.toml` file. It represents the URL of the database. You can modify it according to your system configuration.

## Running it locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [spin](https://developer.fermyon.com/spin/v2/install)
- A PostreSQL database running on your machine (docker or bare metal)

### Add WASM target
```bash
rustup target add wasm32-wasi
```

### Build the app
```bash
spin build 
```

As you can see in api-wasi/spin.toml file, this command is going to build your application using cargo with the target parameter set to `wasm32-wasi` as you can see below :
```toml
[component.hello-rust.build]
command = "cargo build --target wasm32-wasi --release"
```

### Run it (WIP)
```bash
spin up
```

## My experience with WASM in Rust

### First try

After hours of work, the package I was using (actix-web) was not compatible with WASM. I had to find another solution. I found a package called `wasm-bindgen` which allows us to create a WASM module from a Rust project. But the point of this is to create bindings between JS and Rust so that's out of scope.

My second choice was [Wasix](https://wasix.org/) that supports axum (a web application framework). It's a project that aims to provide a WASI runtime for WebAssembly. I tried to use it but I had some issues with the compilation of the project. Following the tutorial [here](https://wasix.org/docs/language-guide/rust/tutorials/wasix-axum), I was able to compile the project but I had an error when I tried to run it.

For documentation pursposes, I used wasmer, a WebAssembly runtime like Wasmtime and Wasmedge.

## Second try

After some research, I found [Spin](https://developer.fermyon.com/spin/v2/index) that provides an SDK for building WebAssembly modules in Rust. I use it to create the API in Rust and then compile it to a WebAssembly module. I was able to create a simple API.

# Author

By Adrien RAIMBAULT
