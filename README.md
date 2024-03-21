# Merkle Tree Storage Server

A basic storage server for String files in which integrity is proven via a Merkle tree proof. 

## Getting started

Install rust. The code has been built and tested using version `1.73.0`.

Build all workspaces:

```bash
  cargo build
```

### Setup and running

The client reads files from directoy `client/files`. The files must have names in the pattern `file0`, `file1`, `file2`, etc.

In one terminal run the server:

```bash
  cd server && cargo run
```

Note the url which it is launched from. It is expected to be `http://127.0.0.1:8000`. If not, then replace this value in the below commands.

In another terminal control the client:

```bash
  cd client
```

First, create a `persist-files` request to persist all files server-side:

```bash
  cargo run -- http://127.0.0.1:8000 persist-files
```

Then request to retrive one of those files:

```bash 
  cargo run -- http://127.0.0.1:8000 retrieve-file $INDEX
```

Where $INDEX is the index of the file you wish to receive, eg 0,1,2 etc.



# Contents

This workspace is made up of 2 libraries, a client and a server.

- `server` is a `Rocket` http server instance which exposes an API to store and retrieve files along with Merkle proofs of their integrity
- `client` is a command line tool which provides commands for using the server's functionality
- `merkle_tree` is a library which implements a Merkle tree complete with proof generation and verification functions
- `simple_database` is a library for writing to the local filesystem
