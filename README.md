# Community Voting dApp on Stellar Soroban

## Project Overview

Community Voting dApp is a simple decentralized voting application built using Stellar Soroban Smart Contracts.

The application allows users to:

* Vote "Yes"
* Vote "No"
* View current voting results
* Reset voting results

All voting data is stored on the Stellar Testnet blockchain using Soroban smart contract storage.

---

# Project Objectives

The purpose of this project is to learn and demonstrate:

* Smart contract development with Rust
* Data storage on Soroban
* Smart contract deployment on Stellar Testnet
* Smart contract interaction through Soroban Studio
* Basic decentralized governance concepts

---

# Technologies Used

## Blockchain

* Stellar Testnet
* Soroban Smart Contracts

## Programming Language

* Rust

## Development Tools

* Soroban Studio
* Freighter Wallet

---

# Smart Contract Functions

## vote_yes()

Adds one vote to the "Yes" counter.

### Example

Before:

```text
Yes = 0
```

After calling:

```text
vote_yes()
```

Result:

```text
Yes = 1
```

---

## vote_no()

Adds one vote to the "No" counter.

### Example

Before:

```text
No = 0
```

After calling:

```text
vote_no()
```

Result:

```text
No = 1
```

---

## get_results()

Returns the current voting results.

### Example Output

```json
[3,2]
```

Meaning:

* Yes Votes = 3
* No Votes = 2

---

## reset()

Resets all voting results back to zero.

### Example Output

```json
[0,0]
```

---

# Data Structure

The smart contract stores two values in Soroban Persistent Storage:

## YesVotes

Stores the total number of "Yes" votes.

## NoVotes

Stores the total number of "No" votes.

Both values remain available on-chain until they are updated or reset.

---

# Test Cases

## Test Case 1: Initial State

Call:

```text
get_results()
```

Expected Output:

```json
[0,0]
```

---

## Test Case 2: First Yes Vote

Call:

```text
vote_yes()
```

Then call:

```text
get_results()
```

Expected Output:

```json
[1,0]
```

---

## Test Case 3: Second Yes Vote

Call:

```text
vote_yes()
```

Then call:

```text
get_results()
```

Expected Output:

```json
[2,0]
```

---

## Test Case 4: First No Vote

Call:

```text
vote_no()
```

Then call:

```text
get_results()
```

Expected Output:

```json
[2,1]
```

---

## Test Case 5: Second No Vote

Call:

```text
vote_no()
```

Then call:

```text
get_results()
```

Expected Output:

```json
[2,2]
```

---

## Test Case 6: Reset Results

Call:

```text
reset()
```

Then call:

```text
get_results()
```

Expected Output:

```json
[0,0]
```

---

# Sample Demonstration

Voting Sequence:

```text
vote_yes()
vote_yes()
vote_yes()
vote_no()
vote_no()
```

Call:

```text
get_results()
```

Output:

```json
[3,2]
```

Result:

```text
Yes Votes = 3
No Votes = 2
```

---

# Conclusion

The Community Voting dApp demonstrates how Soroban Smart Contracts can be used to build a simple decentralized voting system on the Stellar blockchain.

This project showcases:

* Smart contract development using Rust
* On-chain data storage
* Contract deployment on Stellar Testnet
* Contract interaction through Soroban Studio

It serves as a foundation for more advanced governance systems, DAO voting mechanisms, and decentralized decision-making applications.
# Vision

The vision of Community Voting dApp is to make decision-making more transparent, secure, and accessible through blockchain technology. 
Traditional voting systems often rely on centralized organizations that may face trust and transparency challenges. By storing voting results on-chain, every participant can verify outcomes independently. This project demonstrates how decentralized governance can empower communities, student organizations, clubs, and online groups to make fair decisions. In the future, the platform could expand to support proposals, voter authentication, and token-based governance, helping build a more open and trustworthy digital society.