# Satoshi's Palace Secret Template
A template Secret Cosm Wasm contract repository, built for code scalability and readability.

## Build
Build the SecretCosmWasm executable:
```Makefile
make build-mainnet
```

## Test
Run the unit-test suite:
```Makefile
make test
```
Testing is done at the handler level. A sample [test_env](./src/handlers/tests/test_env.rs) has been provided. Testing functions can be added to the test environment to call specific handlers, using the `TestEnv`'s underlieing `deps/info/env`. Seperate testing files are made to test various functionality, and user flows.

## Code Coverage Setup

### Steps to Set Up Code Coverage for Rust Smart Contracts
0. **If the following steps don't work you may need to install `openssl`**
   ```sh
   sudo apt-get update
   sudo apt-get install pkg-config libssl-dev
   ```

1. **Install `cargo-tarpaulin`**:
   ```sh
   cargo install cargo-tarpaulin
   ```

2. **Run the Tests**:
   ```sh
   make test
   ```

3. **Generate the Coverage Report**:
   ```sh
   make coverage
   ```

4. **Open the Coverage Report**:
   ```sh
   make open-coverage
   ```

## Schema
```Makefile
make schema
```
[Schema Generator](./src/bin/schema.rs)

## Code Structure
This codebase is structured to handle Secret-Wasm smart contract projects with growing complexity in an easy to maintain fashion, through the following patterns:

### Handlers
Handlers are high level functions which utlize multiple services to accomplish a specific end user action or query.
 - Execute Handlers return a `StdResult<Response>`.
 - Query Handlers return a `StdResult<Binary>`.

This Format allows for the [contract](./src/contract.rs) to remain readable and scalable.

### Services
Services are mid level functions which provide utility/access functionality for a specific datatype. In general services should only make storage calls for their given datatype, this is a firm rule, however under extraneous circumstances may be broken. If you need to combine the use of multiple datatypes, that should generally be done at the handler level, and not the service level.

Abstracting the utility/access of datatypes with a service layer, allows for a maintainable a readable handler layer; Encouraging different handlers to be able to re-use service level functions.

### Data
Data types are structs which are stored onchain. All struct properties should be private and offer `getter/setter` methods when applicable as well as a `new` method. More complex data types may include impl methods to assert/access/change the data type.

### Error
Errors are `ThisError` implementations which are specific to a datatype. Services and Data types may use Errors of their type to indicate various problems with a method call.

### Msgs & Responses
Msgs are what a user sends to the smart contract to perform different operations on the smart contract.

Responses are what the smart-contract will send back to the user. [ExecuteResponses](./src/responses/execute/execute_response.rs) and [QueryResponses](./src/responses/query/query_response.rs), should be aggregated under an enum.

Why are messages and responses seperated into individual structs? In order to simplify the maintenance of the Contract entry points, individual msgs are help in structs and aggregated under enums, this allows `commands` (individual structs) to be passed to their handlers, Allowing for changes in command paramters to affect only the handler. In addition this makes serialization to json schema and finally into type script a much simpler process.
