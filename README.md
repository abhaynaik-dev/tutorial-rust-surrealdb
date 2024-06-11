
<div align="center">
  <h1>Rust Backend Tutorial</h1>
  <p>
    A tutorial demonstrating how to use Rust in backend development
  </p>
</div>

## :space_invader: Tech Modules

- [Rust](https://www.rust-lang.org/) 🦀
- [Actix Web](https://actix.rs/)
- [SurrealDB](https://surrealdb.com/)
- [AmazonS3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html)
- [Twilio](https://www.twilio.com/en-us)
- [Docker](https://www.docker.com/)

## :pencil: Prerequisites
- Basic knowledge of rust and docker
- A Twilio account; [signup](https://login.twilio.com/u/signup?state=hKFo2SBUQ3dqaC0xYktPTlZRTHQ5UlgtRFVtTWFQVG5hTjF4R6Fur3VuaXZlcnNhbC1sb2dpbqN0aWTZIEtnRXBKUWNBQlZ2VEFtT19CYVFiQ1VsZVVOMGlnaEVko2NpZNkgTW05M1lTTDVSclpmNzdobUlKZFI3QktZYjZPOXV1cks) for free account
- Amazons S3 account; [signup](https://portal.aws.amazon.com/billing/signup?p=pm&c=s3&z=1#/start/email) for free account

## :key: Getting started
- Clone this repository
- Navigate to tutorial-rust-surrealdb folder
- Update .env file with your Twilio and Amazon S3 credentials

## :whale: Build with docker
First build the SurrealDB, -d will run the container in background mode
```bash
docker-compose up -d db
```
Next, run your backend
```bash
docker-compose up
```

## :computer: Output
The successful deployment of rustapp should print
```console
tutorial_rust_surrealdb  |  INFO  rustapp > configuring S3 client
tutorial_rust_surrealdb  |  INFO  rustapp > using AWS region: <AWS_REGION_FROM_ENV>
tutorial_rust_surrealdb  | 🚀 Server started successfully at http://0.0.0.0:8080
```

Sample API testing
```console
➜  tutorial-rust-surrealdb git:(master) ✗ curl http://0.0.0.0:8080/test
"Hello Rustacean 🦀"%
```

## :bookmark: To-Do
- Authentication for SurrealDB
- Deployment on Amazon EC2 instance using docker
- Adding JWT support
- Reducing the rust docker image size
