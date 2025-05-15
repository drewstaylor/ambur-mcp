# Ambur MCP server

MCP server in Rust, for wrapping Ambur query and execute entry point messages to be broadcast by a signer.

### Setting up dependencies
Some dependencies in `Cargo.toml` are provided by private and public GitHub repositories. Building this project requires that all GitHub dependencies must first by cloned to the `./packages` folder and built.

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
