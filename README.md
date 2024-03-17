# Yggdrasil

A vault for keeping your most prized possession: your data. This is an attempt of the [Coding Challenge](https://codingchallenges.substack.com/p/coding-challenge-48-data-privacy).

The service is built using the gRPC architecture, through the use of [Tonic](https://github.com/hyperium/tonic). This allows the service to run on HTTP/2 and protocol buffers to ensure strongly-typed service contracts, data serialisation and code generation in a variety in several programming languages.

## Step 1

Create a public-facing API that:

- [ ] Tokenizes data
- [ ] De-tokenizes data

## Step 2

Create a persistent storage:

- [ ] Create table for token
- [ ] Create table for users

## Step 3

Add authenticated access to the service (needs more work on the breakdown).
