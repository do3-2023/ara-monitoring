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

PS : We can also install ArgoCD to manage the deployment of the project more easily like I did. You can find the installation steps [here](https://argoproj.github.io/argo-cd/getting_started/).

## WASI API

The goal of this part of the project was to reproduce our API in a WebAssembly virtual machine. Because it seems to be a well-suited language for WASM, I chose Rust.

After hours of work, the package I was using (actix-web) was not compatible with WASM. I had to find another solution. I found a package called `wasm-bindgen` which allows us to create a WASM module from a Rust project. But the point of this is to create bindings between JS and Rust so that's out of scope.

My second choice was [Wasix](https://wasix.org/) that supports axum (a web application framework). It's a project that aims to provide a WASI runtime for WebAssembly. I tried to use it but I had some issues with the compilation of the project. Following the tutorial [here](https://wasix.org/docs/language-guide/rust/tutorials/wasix-axum), I was able to compile the project but I had an error when I tried to run it.

For documentation pursposes, I used wasmer, a WebAssembly runtime like Wasmtime and Wasmedge.

## Running it locally

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Wasix](https://wasix.org/docs/getting-started/installation)

### Build the app
```bash
cargo wasix build
```

This command create a .wasm file in the target/wasm32-wasmer-wasi/debug folder.  Using wasmer or the command below, you could potentially run the project, providing you with a Web Assembly micro virtual machine.

### Run it (WIP)
```bash
cargo wasix run
```

# Author
By Adrien RAIMBAULT 