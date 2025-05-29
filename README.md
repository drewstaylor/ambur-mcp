# Ambur MCP server

MCP server in Rust, for wrapping Ambur query and execute entry point messages to be broadcast by a signer.

### Configuring dependencies
Some dependencies in `Cargo.toml` are provided by GitHub repositories, including some repositories which are private. If you don't have read access to the private repositories, unfortunately, you will not be able to build this project. 

This project remains open source as it provides examples for developers building an MCP server for multiple CosmWasm contracts. If you're looking for an example you can build and test yourself check out the [cosmwasm-mcp-template](https://github.com/archway-network/cosmwasm-mcp-template) repository; it shares most of the same code as this repository, but doesn't rely on any private dependencies. The main difference between the two projects being, the [cosmwasm-mcp-template](https://github.com/archway-network/cosmwasm-mcp-template) doesn't include multi-contract support.

### Building this project

To build this project requires the `nightly` build of Rust, this will allow using edition 2024 of rustc.

```sh
# Switch rustc to `nightly` channel
rustup default nightly
```

```sh
# Build for development
cargo build
```

```sh
# Build for deployment
cargo build --release
```

### Tools provided by this MCP server

This MCP server provides the following 15 tools and functionality.

1. `list_contract_deployments` - Lists Ambur core contract addresses (mainnet and testnet)
2. `list_nft_collections` - Lists Ambur NFTs (mainnet and testnet contract addresses, collection name, and collection description)
3. `list_query_entry_points` - Lists the queries that can be made to the core Ambur marketplace contract
4. `list_query_responses` - Lists the detailed response data that can be fetched from the queries that can be made to the core Ambur marketplace contract
5. `build_query_msg` - Build a query to the core Ambur marketplace contract, that can be broadcast by an RPC connected wallet
6. `list_tx_entry_points` - Lists the transactions that can be made to the core Ambur marketplace contract
7. `build_execute_msg` - Build a transaction to the core Ambur marketplace contract, that can be signed and broadcast by an RPC connected wallet
8. `list_token_query_entry_points` - Lists the queries that can be made to an NFT token that can be traded on Ambur (e.g. Archies, Derpies, Foresight and Ghouls)
9. `build_token_query_msg` - Build a query to an NFT token that can be traded on Ambur (e.g. Archies, Derpies, Foresight and Ghouls), that can be broadcast by an RPC connected wallet
10. `list_token_tx_entry_points` - Lists the transactions that can be made to an NFT token that can be traded on Ambur (e.g. Archies, Derpies, Foresight and Ghouls)
11. `build_token_execute_msg` - Build a transaction to an NFT token that can be traded on Ambur (e.g. Archies, Derpies, Foresight and Ghouls), that can be broadcast by an RPC connected wallet
12. `list_minter_query_entry_points` - Lists the queries that can be made to a minter contract for an NFT that can be traded on Ambur (e.g. Archies minter, Derpies minter, Foresight minter and Ghouls minter)
13. `build_minter_query_msg` - Build a query to a minter contract for an NFT token that can be traded on Ambur (e.g. Archies minter, Derpies minter, Foresight minter and Ghouls minter), that can be broadcast by an RPC connected wallet
14. `list_minter_tx_entry_points` - Lists the transactions that can be made to a minter contract for an NFT token that can be traded on Ambur (e.g. Archies minter, Derpies minter, Foresight minter and Ghouls minter)
15. `build_minter_execute_msg` - Build a transaction to a minter contract for an NFT token that can be traded on Ambur (e.g. Archies minter, Derpies minter, Foresight minter and Ghouls minter), that can be broadcast by an RPC connected wallet

### Connecting MCP to Claude Desktop

For default setups, build a release binary and point the mcp server's `command` to its path. No run arguments (`args`) are required:
```js
// claude_desktop_config.json
{
  "mcpServers": {
    "ambur": {
      "command": "/your-computer-path/ambur-mcp/target/release/ambur-mcp",
      "args": []
    }
  }
}
```

For Virtual Machine setups and WSL users, execute the VM as the `command` and use run arguments (`args`) to point the VM where to run the binary:
```js
// claude_desktop_config.json
{
  "mcpServers": {
    "ambur": {
      "command": "wsl.exe",
      "args": [
        "bash",
        "-ic",
        "/your-vm-path/ambur-mcp/target/release/ambur-mcp",
      ]
    }
  }
}
```

### Connecting MCP to LangGraph

[@langchain/mcp-adapters](https://www.npmjs.com/package/@langchain/mcp-adapters) must be installed in the graph project. This package will convert the MCP endpoints into Graph tools.

#### Using @langchain/mcp-adapters

```ts
// graph.ts
import { MultiServerMCPClient } from "@langchain/mcp-adapters";
// ...
// Create client and connect to server
const client = new MultiServerMCPClient({
  throwOnLoadError: true,
  prefixToolNameWithServerName: true,
  additionalToolNamePrefix: "mcp",
  mcpServers: {
    ambur: {
      transport: "stdio",
      command: "~/mcp-servers/ambur-mcp", // path to pre-built linux binary 
                                          // stored in the Graph repo
      args: [],
      restart: {
        enabled: true,
        maxAttempts: 3,
        delayMs: 1000,
      },
    },
  },
});

const tools = await client.getTools();
// ...
```
