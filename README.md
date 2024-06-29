# Serverless Rust API Demo

This is the accompanying repository for the
"Serverless Rust API on AWS" blog post series.

It contains the code for running a Poem-based web API
on AWS Lambda. Deployments are done using AWS CDK.

## Pre-requisites

- A recent version of Rust

## To run the application locally

In order to run the application locally, run

```shell
cargo run --bin local
```

You can use Swagger UI to view the API documentation
at http://0.0.0.0:3000.