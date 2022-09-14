This is the backend which serves a REST-API to interact with the postgres SQL Database.

To build the project run:
  $ cargo build --release

To build & run directly (for debugging):
  $ cargo run

A postgres SQL server is required to use the backend since the data is stored there:
  - Install a postgres server and set the appropriate login settings at the start of the get_database_config() method (this will change in the future)
