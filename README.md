# k8s-book-watcher-rs

POC to test k8s watcher with rust.

Next step is to created an ingress using rust.

## Compiling and running.

Startup a Kubernetes cluster (minikube or kind) and check that your current context
is configured to the testing cluster:

```sh
kubectl config get-contexts 
```

Apply the crd:

```sh
kubectl apply -f deploy/crd.yml
```

Download and compile dependencies and run the watcher:

```sh
cargo run
```

In another terminal add some books:

```sh
kubectl apply -f deploy/books.yml
```