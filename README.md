# Centichain

Centichain is a decentralized blockchain platform built with Rust and Tauri, designed to provide a secure, scalable, and efficient distributed consensus mechanism. This project combines modern blockchain technology with a user-friendly desktop interface to create a robust decentralized network.

## Features

- **Advanced P2P Networking**: Built on libp2p framework for reliable peer discovery, message routing, and network resilience
- **Military-grade Security**: Implements ED25519 cryptographic signatures for secure transaction signing and validation
- **Persistent Storage**: Utilizes MongoDB for efficient block storage and quick data retrieval
- **Dynamic Consensus**: Features a novel rotation-based validator system for fair and secure block production
- **Cross-platform Support**: Built with Tauri framework for lightweight, secure desktop applications on Windows, macOS, and Linux
- **Real-time Synchronization**: Automatic block propagation and chain synchronization across the network
- **Memory-efficient Design**: Optimized memory pool for pending transaction management

## Architecture

The system implements a layered architecture with the following components:

- **Block Generation Module**
  - Handles block creation and validation
  - Implements transaction verification
  - Manages block timestamps and sequencing
  - Ensures cryptographic integrity of blocks

- **Network Layer**
  - Manages peer discovery and connections
  - Implements efficient message propagation
  - Handles network partitioning and recovery
  - Provides NAT traversal capabilities

- **Consensus Engine**
  - Coordinates validator rotation mechanism
  - Ensures network-wide agreement on block finality
  - Handles fork resolution and chain selection
  - Implements Byzantine fault tolerance

- **Storage Layer**
  - Provides persistent blockchain storage using MongoDB
  - Implements efficient indexing and querying
  - Manages state transitions and updates
  - Handles chain reorganizations

- **Desktop Interface**
  - Offers intuitive blockchain interaction
  - Provides real-time network statistics
  - Includes wallet management features
  - Displays detailed transaction history

## Getting Started

### Prerequisites

- Rust toolchain (1.70 or later)
- Node.js (v16 or later) and npm
- MongoDB (v6.0 or later)
- Tauri CLI (latest version)
- Cargo (latest version)
- Build essentials for your OS

### Installation

1. Clone the repository:
