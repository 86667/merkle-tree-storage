# Storage Server with Integrity Proof

We build a simple service with the following requirements:

- A client should be able to store a list of files and retrieve any one of those files by its index in the list
- Upon storage the client should be returned a Merkle Tree root hash
- Upon retrieval of a file the client should also be returned a Merkle Proof which is used to validate that the retrieved file has not be modified since the client stored it

See intructions for running the code in the [README](README.md).


## What it can do

The code manages the above requirements for basic String files. We implement a Merkle Tree library for contruction of the tree and generation of proofs. The Server provides an API for storing and retrieving files along with their Merkle proofs, and a client calls the server's API and verifies the given proofs.

I left the burden of performing the hashing of files to the client and passing the resulting digests along with the files. This was so that decisions around file structure and whether a hashing algorithm is deemed secure are left up to the client. The server simply takes some files, stores them and uses the hashes provided by the client to build it's Merkle tree.

## Improvements required for production-ready

There are a number of obvious places in which the code falls very short from proudciton-ready. Here are a few items which would need to be resolved before any kind of release: 

- We currently `panic` for both caught user errors and unexpected errors. Some `ServerError` type should be defined with user friendly return errors for all possible failure scenarios.
- We send files as `Vec<String>` in the body of a http `post` request. This is not the tool for the job, we should use a file server protocol


## Efficiecy improvements

We persist the list of hashes and rebuild the Merkle tree for each file retrieval. This is a half-way house between the extremes of: 

1) Persiting only the files
2) Persisting the files and the constructed Merkle tree 

This is ultimately a trade-off between storage and CPU constraints. 

Option 1) may be a better choice if we are certain that all files will be sized such that they do not require a huge number of serial operations within the hash function, such as would be the case for sha256 for files in the gigabytes. Or, we may instead use a parallelisable hash function to solve this if memory is not a tight resource.

Option 2) may be the better choice if the server must be restrictive on storage or processing power. If a Merkle tree takes a while to generate because of resources then it may be better to have the client wait on the `store()` call rather than on the `fetch()` call.


## Security improvements

- hash fn used. why sha256? may want to update to sha3 in a few years. how to manage this
- potenial for attack (See Second preimage attack)

## Scalability 

- how to replicate to multiple machines 
  - need some domain managment service to route
  - master-slave model or fancy distributed storage algorithm with locking

## Usability improvements

- Do not constrain number of files to be a power of 2
- encryption metadata - algo, key used to encrypt, naming of files rather than index
- users and authentication
- Storage of just one set of files
- Edit files capability