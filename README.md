# ferris

An API implementation for, Ferris - The Coaching Assistant.

## Technology stack:

Ferris is a RUST implementation

Actix is the Web framework
Juniper is for GraphQL
Diesel is for Connection Pooling and Database interaction
MySql is the database server

We are thankful to the open source community for rapid development

## Building the Server:

0. Clone this respository

1. Hope you have mysql installed (else "brew install mysql")
2. Hope you have rust and cargo installed.
3. Ensure to install diesel_cli (cargo install diesel_cli)
4. Change the user and password for accessing the mysql server at .env
5. Run "diesel setup" 
    5.1 This should create the database - Ferries
    5.2 This should run the migrations to create tables

6. cargo run

This web-server is configured to start at localhost:8088


## Testing the API

Access http://localhost:8088/graphiql from your browser


## The Web-UI

The Web-UI, that uses the services of this web-server, is accessible from https://krscode.com


## Thanks

This is a work-in-progress and is scheduled for an Alpha testing by Aug 2020.

Please write to krsmanian1972@gmail.com

Thanks!