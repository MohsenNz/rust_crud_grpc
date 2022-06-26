## Application Design

The application itself has been designed with a modular approach. Dividing this project into multiple modules, and many small files was intentional, as it makes the overall architecture clear.

File structre inspired by nestjs.

  ```
  proto
  └── sensor.proto
  src
  ├── error.rs
  ├── lib.rs
  ├── sensor
  │   ├── dao.rs
  │   ├── mod.rs
  │   ├── model.rs
  │   └── service.rs
  └── bin
      ├── client.rs
      └── server.rs
  ```

- `main.rs`, tried to keep simple. it's contain Initialize of other modules, and run http-server.
- `lib.rs` expose API for all modules.
- `error.rs` uniform all error in projects.
- `config.rs` read configuration from environment (`.env` exists for conveniences).
- `sensor`, each .proto is know as module and will configure separately. (like `sensor`).
  - `dao.rs`, stand for Data-Access-Object, means routines for access and modify database objects.
  - `model.rs`, contain database objects maps to rust struct.
  - `mod.rs`, contain resource configuration.
  - `service.rs`, contain proto API definition.
- `bin`
  - `client` client side program.
  - `server` server side program.

## Test Coverage

This application uses an unit test. These test serve as an example for what is sufficient test coverage for an initial application.

## Setup

1. Create database user

  ```shell
  createuser -P postgres
  ```

  Enter a password of your choice. The following instructions assume you used `123456` as password.

  This step is **optional** and you can also use an existing database user for that. Just make sure to replace `postgres` by the database user of your choice in the following steps and change the `.env` file containing the configuration accordingly.

  `DATABASE_URL=postgres://<database_user>:<password>@<host>/<database_name>`

2. Create database

  ```shell
  createdb -O postgres rust_crud_grpc
  createdb -O postgres rust_crud_grpc_test
  ```

3. Initialize database

  ```shell
  psql -f schema.sql rust_crud_grpc
  ```

4. Rename `.env.sample` to `.env`.

5. Run the server:

  ```shell
  cargo run --bin server
  ```
6. Use [bloomrpc](https://github.com/bloomrpc/bloomrpc/blob/master/README.md) client to test API.

7. Run the tests:

  ```shell
  cargo test
  ```
