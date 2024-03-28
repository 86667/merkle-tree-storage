# Storage Server with Integrity Proof

We build a simple service with the following requirements:

- A client should be able to store a list of files and retrieve any one of those files by its index in the list
- Upon storage the client should be returned a Merkle Tree root hash
- Upon retrieval of a file the client should also be returned a Merkle Proof which is used to validate that the retrieved file has not be modified since the client stored it
- An easy-to-run demo of the above functionality 

See intructions for running the code in the [README](README.md).


## What it can do

The code manages the above requirements for basic String files. We implement a Merkle Tree library for contruction of the tree and generation of proofs. The Server provides an API for storing and retrieving files along with their Merkle proofs, and a client calls the server's API and verifies the given proofs.

I left the burden of performing the hashing of files to the client which passes the resulting hash digests along with the files. This was so that decisions around file structure and whether a hashing algorithm is deemed secure are left up to the client. The server simply takes some files, stores them and uses the hash digests provided by the client to build it's Merkle tree.


## Improvements required for production-ready

There are a number of obvious places in which the code falls very short from proudciton-ready. Here are a few items which would need to be resolved before any kind of release: 

- We currently `panic` for both caught user errors and unexpected errors in both client and server. Some `ServerError` type should be defined with user friendly return errors for all possible failure scenarios
- We send files as `Vec<String>` in the body of a http `post` request. This is not the tool for the job - a file transfer protocol should be used to avoid hitting size limits 
- Hosting capabilities are not included here along with network safety mechanisms such as ddos protection, firewall, authentication etc
- Lots of functionality is left untested and so there are most likley many bugs. All fucntions containing logic should be tested with network and db calls mocked. 

## Efficiecy improvements

We persist the list of hashes and rebuild the Merkle tree for each file retrieval. This is a half-way house between the extremes of: 

1) Persiting the files only
2) Persisting the files and the constructed Merkle tree 

This is ultimately a trade-off between storage and CPU constraints. 

Option 1) may be a better choice if we are certain that all files will be sized such that they do not require a huge number of serial operations within the hash function, such as would be the case for sha256 for files in the gigabytes. Or, we may instead use a parallelisable hash function to solve this if memory is not a tight resource.

Option 2) may be the better choice if the server must be restrictive on storage or processing power. If a Merkle tree takes a while to generate because of resources then it may be better to have the client wait on the `store()` call rather than on the `fetch()` call.


## Security improvements

The security of our Merkle proof is ultimately dependent on the collision resistence of our chosen hash function. An adversarial server with the capability to produce a second pre-image for any of the hash digests in the tree could produce a different file and a proof which still verifies with the same root hash.

One way to mitigate this would be to return the tree depth along with the root hash on creation. The verifier would then check that a proof has the correct number of hash digests. This would limit the adversary to having to produce a second pre-image for the exact file that was requested for retreival.

A further enhancement for the most security concernced would be to use several hash functions in constructing several Merkle trees, roots and proof. Finding a second pre-image for a given hash function is one thing but finding a second pre-image for some data over multiple hash functions is about as inplausible as we can hope to achieve.


## Usability improvements

Some usability improvements:

- Metdata - We may wish to allow the user to store some metadata along with their files. A name which can be used to reference it would be useful rather than requiring its index. Encryption data such as algorithm and a reference to which key to decrypt with so that this doesnt need to be stored locally by the client may also be handy. 
- Our server currently can store only one set of files. This should be expanded to store a set of files for distinct users with authentication, perhaps via api keys, to limit access.
- Files are written as a whole and read one-by-one. It would be fairly straightforward to allow for files to be edited and a new root hash returned, and for retrieval to be done mutliple files at a time.


## Scalability 

In order for this server code to scale the first thing required would be for the database to be replaced with some form of distributed solution. Currently, each instance of the server and its stored data would be siloed. There would also need to be some API management system to handle requests from the central domain entry point and forward them to the server instances.
