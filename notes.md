## TODO

- [ ] Client - Server 
  - [ ] Build both code bases and compile with basic command
  - [ ] Have shared lib imported
  - [ ] Run both and interact successfully
- [ ] Merkle tree implementation
  - [ ] lib class interface. provide hashes better than data? user of lib can define hwo the data should be manipulated
  - [ ] implement proof generation
- [ ] Server storage
  - [ ] storage interface to store eg {id, hash, blob}
  - [ ] implement basic mongodb storage
- [ ] Client code
  - [ ] cli fn for upload which returns merkle root calculated locally. check server return root is the same
  - [ ] cli fn for get which takes filename and merkle root


## Considerations

- Priorities of server - speed of access (precompute merkle proof) or storage size (computer merkle proof on the fly)
- Hashed contents of file discussion. what exactly gets hashed, include modified date?
- Merkle tree
  - hash fn used. why sha 2?
  - potenial for attack (See Second preimage attack)
  - odd number of elements?
- Editing: updating file's content fn
- over the network and replication
  - how to make accessible over the network
  - how to replicate to multiple machines 
    - need some domain managment service to route
    - master-slave model or fancy distributed storage algorithm with locking
- production extra thigns to consider:
  - users and authentication
  - encryption metadata - algo, key used to encrypt


## merkle tree

### naive

- Find merkle root
  - Build row 0: hash each input element
  - Build row 1: hash each 2 items
  - Build row 2: hash each 2 items
  - .. until row length is 1
- Prove data hash, with index
  - Find data hash for index
  - Find data hash for index + 1 if even, -1 if odd
  - 