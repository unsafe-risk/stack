# stack

stack is a simple project structure and management tool for backend projects.

## Installation

```bash
cargo install --git https://github.com/unsafe-risk/stack
```

## Project Structure

### Initialize

#### go

```bash
stk init <project-name> --go
```

#### rust

```bash
stk init <project-name> --rust
```

### Common Module

- **Message**
  - The data structure used within the app
    - Value Object
- **Model**
  - The data structure to be exchanged with external clients
    - Data Transfer Object
  - The data structure used for external requests
    - Data Access Object
  - Supports JSON, protobuf, messagepack, etc.
- **State**
  - The state values that affect the operation of the app globally
  - Used to store configuration values
  - Replaces global variables
- **Protocol**
  - Logic for validating data
  - Defines communication protocols
- **Service**
  - Owns an independent state
  - Capable of making requests to external services or servers
- **Contract**
  - The form of API that should be provided
  - Composed in the form of `interface` or `trait`
  - Corresponds to `Service` and `Aggregator`

### Business Module

- **Mediator**
  - Independent threads are created
  - Receives messages asynchronously
  - Sends messages asynchronously
  - Exists for communication between various components
- **Aggregator**
  - An integrated API consisting of multiple services
- **Handler**
  - The action to process a certain request
  - Needs to be generalized to handle requests through any communication protocol
- **Adapter**
  - The role of converting requests that come in through a certain communication protocol to be delivered to the handler
  - The implementation is specific to a particular handler and communication protocol
- **Server**
  - The role of receiving external requests through a specific communication protocol
  - Can have adapters, which call the handlers

### Generate module

Generate a module with the following command.

```bash
stk generate <module-path> <module-name> --service
```

To view the help, use the following command.

```bash
stk generate --help
```

## TODO

- [ ] Add `--buf` option to `stack init` command
- [ ] Add `--proto` option to `stack generate` command
- [ ] Implement dependency management for `rust` projects
- [ ] Implement `stack init --rust` command
