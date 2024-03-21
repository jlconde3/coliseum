# COLISEUM NET

This Rust project implements a distributed system where nodes communicate with each other over TCP connections and handle requests from clients. It provides functionality for creating, retrieving, and distributing items among nodes in the network.

## Features

**Communication**: Nodes communicate with each other and with clients over TCP connections using JSON serialization.
**Item Management**: Nodes can create and retrieve items, which are stored locally and can be distributed to other nodes in the network.
**Scalability**: The system is designed to scale by allowing multiple nodes to interact with each other concurrently.
