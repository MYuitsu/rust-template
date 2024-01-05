Rust template
> **A lightweight and flexible project template built with [cargo generate](https://github.com/ashleygwilliams/cargo-generate). This template is designed to simplify the creation of projects that apply a Vertical Slice Architecture.**

# Table of Contents
- [Table of Contents](#table-of-contents)
  - [Quick Start](#quick-start)
    - [Create a new project](#create-a-new-project)
    - [Installation](#installation)
  - [Structure of Project](#structure-of-project)
  - [The Goals of This Project](#the-goals-of-this-project)
    - [Monolithic application](#monolithic-application)
  - [Plan](#plan)
  - [Technologies - Libraries](#technologies---libraries)
  - [Testing](#testing)
    - [Gathering code coverage](#gathering-code-coverage)
  - [Project References \& Credits](#project-references--credits)
  - [License](#license)

## Quick Start

```sh
cargo install cargo-generate
```
### Create a new project
```sh
cargo generate --git https://github.com/MYuitsu/rust-template
```
Answer the prompts and a new project will be created.

### Installation

- Install [Rust](https://www.rust-lang.org/tools/install)
- Run `cargo run` to build and run service
- Install `cargo-make`
```sh
  cargo install cargo-make
```
<!-- - Run installation task ( install dependencies )
- 
```sh
  cargo make install-deps
``` -->

<!-- ### Install Docker and Docker Compose (Optional)

- [Docker](https://docs.docker.com/engine/install/)
- [docker-compose](https://docs.docker.com/compose/install/) -->

## Structure of Project
<pre>
project_root/
│
├── src/
│   ├── main.rs                             // Entry point for the application
│   ├── app.rs                              // Application setup and configuration
│   ├── config.rs                           // Configuration-related structures and functions
│   ├── slices/                             // Vertical slices of the application
│   │   ├── feature_x/                      // A feature slice (e.g., users, orders, etc.)
│   │   │   ├── command.rs                  // CQRS commands for the feature
│   │   │   ├── query.rs                    // CQRS queries for the feature
│   │   │   ├── endpoint.rs                 // Axum endpoints for the feature
│   │   │   ├── model.rs                    // Domain models for the feature
│   │   │   ├── service.rs                  // Business logic for the feature
│   │   │   ├── validators.rs               // Fluent validation logic
│   │   │   └── tests.rs                    // Tests for the feature
│   │   └── feature_y/                      // Another feature slice
│   │
│   ├── infrastructure/                     // Infrastructure concerns
│   │   ├── migrations/                     // Database migrations
│   │   │   ├──yyyyddmm-hhMMss-create.sql   // Migration SQL script
│   │   ├── database.rs                     // Database connection and management
│   │   ├── messaging.rs                    // Integration for authentication
│   │   ├── messaging.rs                    // Inbox/Outbox pattern implementation
│   │   ├── monitoring.rs                   // Monitoring setup (e.g., Prometheus, Grafana)
│   │   └── ...
│   │
│   ├── middleware/                         // Custom middleware for logging, tracing, etc.
│   │   ├── logging.rs
│   │   ├── tracing.rs
│   │   ├── metrics.rs                      // Custom middleware for metrics collection
│   │   └── ...
│   │
│   └── utils/                              // Utility modules
│       └── ...
│
├── Cargo.toml                              // Rust package manifest
├── Makefile.toml                           // Makefile for project
├── Cargo.lock                              // Lock file for dependencies
└── .env                                    // Environment variables for local development
</pre>

## The Goals of This Project

### Monolithic application

- [ ] Using `Vertical Slice Architecture` for `architecture` level.
<!-- - :sparkle: Using `Domain Driven Design (DDD)` to implement all `business processes` in microservices.
- :sparkle: Using `Rabbitmq` on top of `Masstransit` for `Event Driven Architecture` between our microservices.
- :sparkle: Using `gRPC` for `internal communication` between our microservices.
- :sparkle: Using `CQRS` implementation with `MediatR` library.
- :sparkle: Using `Postgres` for `write side` of some microservices.
- :sparkle: Using `MongoDB` for `read side` of some microservices.
- :sparkle: Using `Event Store` for `write side` of Booking-Microservice to store all `historical state` of aggregate.
- :sparkle: Using `Inbox Pattern` for ensuring message idempotency for receiver and `Exactly once Delivery`. 
- :sparkle: Using `Outbox Pattern` for ensuring no message is lost and there is at `At Least One Delivery`.
- :sparkle: Using `Unit Testing` for testing small units and mocking our dependencies with `Nsubstitute`.
- :sparkle: Using `End-To-End Testing` and `Integration Testing` for testing `features` with all dependencies using `testcontainers`.
- :sparkle: Using `Fluent Validation` and a `Validation Pipeline Behaviour` on top of `MediatR`.
- :sparkle: Using `Minimal API` for all endpoints.
- :sparkle: Using `Health Check` for `reporting` the `health` of app infrastructure components.
- :sparkle: Using `Docker-Compose` and `Kubernetes` for our deployment mechanism.
- :sparkle: Using `Kibana` on top of `Serilog` for `logging`.
- :sparkle: Using `OpenTelemetry` for distributed tracing top of `Jaeger`.
- :sparkle: Using `OpenTelemetry` for monitoring top of `Prometteuse` and `Grafana`.
- :sparkle: Using `IdentityServer` for authentication and authorization base on `OpenID-Connect` and `OAuth2`.
- :sparkle: Using `Yarp` as a microservices `gateway`.
- :sparkle: Using `Kubernetes` to achieve efficient `scaling` and ensure `high availability` for each of our microservices.
- :sparkle: Using `Nginx Ingress Controller` for `load balancing` between our microservices top of `Kubernetes`.
- :sparkle: Using `cert-manager` to Configure `TLS` in `kubernetes cluster`. -->

## Plan

This project is a work in progress

| Feature           | Status         |
| ----------------- | -------------- |
| Connect to DB     | In Progress    |




<!-- // TODO -->

<!-- > 🌀This project is a work in progress, new features will be added over time.🌀

I will try to register future goals and additions in the [Issues](https://github.com/meysamhadeli/booking-microservices/issues) section of this repository.

High-level plan is represented in the table

| Feature           | Status         |
| ----------------- | -------------- |
| API Gateway       | Completed ✔️   |
| Identity Service  | Completed ✔️   |
| Flight Service    | Completed ✔️   |
| Passenger Service | Completed ✔️   |
| Booking Service   | Completed ✔️   |
| Building Blocks   | Completed ✔️   | -->

## Technologies - Libraries

cargo-make

-  **[`cargo-make`](https://github.com/sagiegurari/cargo-make)** - Build automation tool for Cargo projects.
-  **[`axum`](https://github.com/tokio-rs/axum)** - A web application framework that focuses on ergonomics and modularity.
-  **[`dotenv`](https://github.com/dotenv-rs/dotenv)** - Library for loading environment variables from `.env` files.
<!-- - ✔️ **[`MVC Versioning API`](https://github.com/microsoft/aspnet-api-versioning)** - Set of libraries which add service API versioning to ASP.NET Web API, OData with ASP.NET Web API, and ASP.NET Core
- ✔️ **[`EF Core`](https://github.com/dotnet/efcore)** - Modern object-database mapper for .NET. It supports LINQ queries, change tracking, updates, and schema migrations
- ✔️ **[`Masstransit`](https://github.com/MassTransit/MassTransit)** - Distributed Application Framework for .NET.
- ✔️ **[`MediatR`](https://github.com/jbogard/MediatR)** - Simple, unambitious mediator implementation in .NET.
- ✔️ **[`FluentValidation`](https://github.com/FluentValidation/FluentValidation)** - Popular .NET validation library for building strongly-typed validation rules
- ✔️ **[`Swagger & Swagger UI`](https://github.com/domaindrivendev/Swashbuckle.AspNetCore)** - Swagger tools for documenting API's built on ASP.NET Core
- ✔️ **[`Serilog`](https://github.com/serilog/serilog)** - Simple .NET logging with fully-structured events
- ✔️ **[`Polly`](https://github.com/App-vNext/Polly)** - Polly is a .NET resilience and transient-fault-handling library that allows developers to express policies such as Retry, Circuit Breaker, Timeout, Bulkhead Isolation, and Fallback in a fluent and thread-safe manner
- ✔️ **[`Scrutor`](https://github.com/khellang/Scrutor)** - Assembly scanning and decoration extensions for Microsoft.Extensions.DependencyInjection
- ✔️ **[`Opentelemetry-dotnet`](https://github.com/open-telemetry/opentelemetry-dotnet)** - The OpenTelemetry .NET Client
- ✔️ **[`DuendeSoftware IdentityServer`](https://github.com/DuendeSoftware/IdentityServer)** - The most flexible and standards-compliant OpenID Connect and OAuth 2.x framework for ASP.NET Core
- ✔️ **[`EasyCaching`](https://github.com/dotnetcore/EasyCaching)** - Open source caching library that contains basic usages and some advanced usages of caching which can help us to handle caching more easier.
- ✔️ **[`Mapster`](https://github.com/MapsterMapper/Mapster)** - Convention-based object-object mapper in .NET.
- ✔️ **[`Hellang.Middleware.ProblemDetails`](https://github.com/khellang/Middleware/tree/master/src/ProblemDetails)** - A middleware for handling exception in .Net Core
- ✔️ **[`NewId`](https://github.com/phatboyg/NewId)** - NewId can be used as an embedded unique ID generator that produces 128 bit (16 bytes) sequential IDs
- ✔️ **[`Yarp`](https://github.com/microsoft/reverse-proxy)** - Reverse proxy toolkit for building fast proxy servers in .NET
- ✔️ **[`Tye`](https://github.com/dotnet/tye)** - Developer tool that makes developing, testing, and deploying microservices and distributed applications easier
- ✔️ **[`gRPC-dotnet`](https://github.com/grpc/grpc-dotnet)** - gRPC functionality for .NET.
- ✔️ **[`EventStore`](https://github.com/EventStore/EventStore)** - The open-source, functional database with Complex Event Processing.
- ✔️ **[`MongoDB.Driver`](https://github.com/mongodb/mongo-csharp-driver)** - .NET Driver for MongoDB.
- ✔️ **[`xUnit.net`](https://github.com/xunit/xunit)** - A free, open source, community-focused unit testing tool for the .NET Framework.
- ✔️ **[`Respawn`](https://github.com/jbogard/Respawn)** - Respawn is a small utility to help in resetting test databases to a clean state.
- ✔️ **[`Testcontainers`](https://github.com/testcontainers/testcontainers-dotnet)** - Testcontainers for .NET is a library to support tests with throwaway instances of Docker containers.
- ✔️ **[`K6`](https://github.com/grafana/k6)** - Modern load testing for developers and testers in the DevOps era. -->
- **[`testcontainers`](https://github.com/testcontainers/testcontainers-rs)** - Testcontainers for Rust
- **[`grcov`](https://github.com/mozilla/grcov)** - Code coverage tool for Rust projects.


## Testing

### Gathering code coverage

```sh
  cargo make coverage
```

## Project References & Credits

- [https://github.com/meysamhadeli/booking-microservices](https://github.com/meysamhadeli/booking-microservices)

## License
This project is made available under the MIT license. See [LICENSE](https://github.com/meysamhadeli/booking-microservices/blob/main/LICENSE) for details.