// Contract addresses
pub static CONTRACT_MAINNET: &str = "archway13s2wjcgx4pxwq3kw66669jt8l4jm840hhhxk7ktpj7r98wc5hvnqraj6w3";
pub static CONTRACT_CONSTANTINE: &str = "archway1y9twsp7sf4ae8jl5huhduv0lwxa0l4g4n9lyvnvjkp648udpwh0q9vycz4";

// System Instructions
pub static SERVER_INFO_DESCR: &str = r#"
This MCP server provides tools for aiding with queries and transactions to 
the Ambur Marketplace contract on Archway Network. It does not broadcast these 
queries or txs, nor does it sign the txs. 

It allows users to perform the following actions: 
- List the available Archway contract addresses, and associated network, where the 
Ambur marketplace contract is deployed ('list_contract_deployments')
- List the available query entry points, and any parameters required for building 
them ('list_query_entry_points')
- Build a query message that can be broadcast by any RPC enabled tool 
('build_query_msg')
- List the available execute (tx) entry points, and any parameters required for 
building them ('list_tx_entry_points')
- Build an execute message (tx message) that can be signed and broadcast by any 
RPC enabled tool with wallet signing capabilities
"#;

// Tool descriptions
// Network
pub static LIST_CONTRACTS_DESCR: &str = r#"
Call this tool to get a list of contract addresses, and associated Archway Network, 
where the Ambur marketplace contract has been deployed. This tool is helpful for 
discovering the mainnet and testnet contract addresses for the Ambur marketplace 
smart contract."#;

// Query
pub static LIST_QUERY_ENTRY_POINTS_DESCR: &str = r#"
Call this tool to get a list of possible queries that can be made (e.g. query entry 
points) to the Ambur marketplace contract, as well as their associated calling 
parameters. This tool is helpful for discovering what parameters a user must 
provide in order to build a prepared query message for a query to the Ambur NFT 
marketplace smart contract."#;

pub static BUILD_QUERY_MSG_DESCR: &str = r#"
Call this tool to build a prepared query message for a query to the Ambur NFT 
marketplace contract. This tool won't broadcast the query or return the query 
result, but can be combined with any RPC connected query tool that accepts a 
well-formed QueryMsg variant for any valid Ambur marketplace contract query entry 
point."#;

// Execute
pub static LIST_TX_ENTRY_POINTS_DESCR: &str = r#"
Call this tool to get a list of possible transactions that can be made (e.g.  
execute entry points) to the Ambur marketplace contract, as well as their 
associated calling parameters. This tool is helpful for discovering what parameters 
a user must provide in order to build a perpared execute message for a tx to the 
Ambur NFT marketplace smart contract."#;

//Prepare a transaction message for signing and RPC broadcast
pub static BUILD_EXECUTE_MSG_DESCR: &str = r#"
Call this tool to build a prepared execute message for a transaction to the Ambur 
NFT marketplace contract. This tool won't sign the message, or broadcast it to the 
blockchain, but can be combined with any RPC connected tx tool that accepts a well-
formed ExecuteMsg variant for any valid Ambur marketplace contract execute entry 
point."#;