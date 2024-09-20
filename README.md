# Match Cosmos Marketplace Contract

This is a CosmWasm-based smart contract for creating and managing a decentralized marketplace. Buyers and sellers can interact via requests and offers to exchange goods or services securely.

## Features

- **User Profiles**: Register, update, and manage user profiles.
- **Store Creation**: Sellers can create stores for buyers to browse.
- **Requests & Offers**: Buyers can create requests, and sellers can respond with offers.
- **Offer Acceptance**: Buyers can accept offers and proceed with transactions.
- **Lifecycle Management**: Requests and offers follow a lifecycle (Pending, Accepted, Locked, Completed).

## Contract Architecture

### State Variables

- **`User`**: Stores user details like username, phone number, account type (Buyer/Seller), and location.
- **`Store`**: Represents a seller’s store.
- **`Request`**: Represents a product or service request from a buyer.
- **`Offer`**: Represents an offer from a seller in response to a buyer's request.

### Execute Messages (`ExecuteMsg`)

- `CreateUser`: Register a user with details like username, phone, and account type.
- `UpdateUser`: Update user profile information.
- `CreateStore`: Sellers create a store with details like name, description, and location.
- `CreateRequest`: Buyers create a request for goods or services.
- `CreateOffer`: Sellers respond to requests with offers.
- `AcceptOffer`: Buyers accept offers to lock the request.
- `DeleteRequest`: Buyers delete their pending requests.
- `ToggleLocation`: Enable or disable location tracking.
- `MarkRequestAsCompleted`: Confirm request completion by the buyer.

### Query Messages (`QueryMsg`)

- `GetUser`: Retrieve user information by address.
- `GetRequest`: Get details of a specific request.
- `GetAllRequests`: Fetch all marketplace requests.
- `GetOffer`: Get details of a specific offer.
- `GetOffersByRequest`: Get all offers for a specific request.
- `GetUserStores`: Get all stores created by a user.
- `GetSellerOffers`: Fetch all offers made by a seller.

## State Counters

- **`USER_COUNT`**: Tracks the total number of users.
- **`STORE_COUNT`**: Tracks the number of stores.
- **`REQUEST_COUNT`**: Tracks the number of requests.
- **`OFFER_COUNT`**: Tracks the number of offers.

---

## Installation

Follow the steps below to build, test, and deploy the CosmWasm contract.

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- CosmWasm CLI tools (`xion`, `wasmcli`) [xiond](https://docs.burnt.com/xion/nodes-and-validators/run-a-node/build-the-xion-daemon)

### 1. Install Rust and Set the Default Toolchain

```bash
rustup default stable
rustup update stable
```

### 2. Add WASM Compilation Target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Required Cargo Packages

```bash
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

### 4. Clone the Repository

```bash
git clone https://github.com/kingsmennn/match-contract-cosmos
cd match-contract-cosmos
```

### 5. Build the Contract

```bash
cargo wasm
```

### 6. Run Tests

```bash
cargo test
```

---

## Deployment

To deploy the contract on a CosmWasm-compatible blockchain (e.g., xion):

### 1. Store the Contract on the Blockchain

```bash
RES=$(xiond tx wasm store target/wasm32-unknown-unknown/release/match_cosmos_contract.wasm \
    --chain-id xion-local-testnet-1 \
    --gas-adjustment 1.3 \
    --gas-prices 0uxion \
    --gas auto \
    -y --output json \
    --chain-id xion-testnet-1 \
    --node https://rpc.xion-testnet-1.burnt.com:443 \
    --from test)
```

### 2. Install JQ (Optional for JSON Processing)

- **Linux**:
  ```bash
  sudo apt-get install jq
  ```
- **Mac**:
  ```bash
  brew install jq
  ```

### 3. Get the Contract Code ID

```bash
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
```

### 4. Instantiate the Contract

Initialize the contract with the desired state:

```bash
INIT='{}'
```

Instantiate the contract:

```bash
xiond tx wasm instantiate $CODE_ID "$INIT" \
    --from wallet --label "my first contract" --gas-prices 0.025uxion --gas auto --gas-adjustment 1.3 -b block -y --no-admin
```

### 5. Get the Contract Address

```bash
CONTRACT_ADDR=$(xiond query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]' -node https://rpc.xion-testnet-1.burnt.com:443 --chain-id xion-testnet-1)
```

---

## Error Handling

The contract uses custom error types for handling exceptions, such as:

- `MarketplaceError::OnlySellersAllowed`: Triggered when a non-seller performs seller-only actions.
- `MarketplaceError::OfferAlreadyAccepted`: Triggered when a buyer tries to accept an already accepted offer.
- `MarketplaceError::UnauthorizedBuyer`: Triggered when a user tries to delete someone else's request.

---

## Contributing

We welcome contributions to improve this contract. To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with a detailed description of the changes.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
