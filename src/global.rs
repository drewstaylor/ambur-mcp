// Contract addresses
pub static CONTRACT_MAINNET: &str =
    "archway13s2wjcgx4pxwq3kw66669jt8l4jm840hhhxk7ktpj7r98wc5hvnqraj6w3";
pub static CONTRACT_CONSTANTINE: &str =
    "archway1y9twsp7sf4ae8jl5huhduv0lwxa0l4g4n9lyvnvjkp648udpwh0q9vycz4";

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
marketplace smart contract.

The response provided from this tool is a JSON schema for the QueryMsg enum of the 
Ambur maketplace contract. It would be too verbose to provide it to your chat 
partner, so summarizing it will be crucial. 

Below is some documentation to help you parse and understand the fields and 
structure of the generated QueryMsg schema, so that you can extrapolate the possible 
queries that can be made to the Ambur smart contract (some non-relevant fields are 
excluded from the documentation and can be safely ignored):

```DOCUMENTATION_BEGIN
* "$schema": a metadata field clarifying which version of JSON schema was used for 
generating this schema

* "title": a field that clarifies what item was serialized into this JSON schema 
(e.g. in this case the 'QueryMsg' enum from the Ambur marketplace smart contract)

* "oneOf": an array of all queries that can be made to the Ambur marketplace smart 
contract, as well as their calling parameters.

* Items of the 'oneOf' array are objects with the following properties:
    - "description": the 'description' sub-field of 'oneOf' items provides a human 
    readable description of what data can be fetched by the query
    - "required": the 'required' sub-field of 'oneOf' items is an array containing 
    a single value. The single value contained by this array is the name of a 
    valid query that can be made to the Ambur marketplace contract
    - "properties": the 'properties' sub-field of 'oneOf' items provides an object 
    representing the query and any required and/or optional calling 
    parameters where: the first sub-field is the name of the query, and any 
    optional or required calling parameters for the query, are contained in a second- 
    level "properties" subfield (e.g. `oneOf[0].properties?.properties`)

    Here's an example "oneOf" item for the 'list_allowed_payments' query, that 
    has no required calling parameters:
    {
      "description": "Lists all allowed payments",
      "type": "object",
      "required": [
        "list_allowed_payments"
      ],
      "properties": {
        "list_allowed_payments": {
          "type": "object"
        }
      },
      "additionalProperties": false
    }

    Here's an example "oneOf" item for the 'list_collection_offers' query, that 
    has optional calling parameters (e.g. for 'limit' and 'start_after'):
    {
      "description": "Get all Collection Offers (enumerable) Return type: ListResponse",
      "type": "object",
      "required": [
        "list_collection_offers"
      ],
      "properties": {
        "list_collection_offers": {
          "type": "object",
          "properties": {
            "limit": {
              "type": [
                "integer",
                "null"
              ],
              "format": "uint32",
              "minimum": 0.0
            },
            "start_after": {
              "type": [
                "string",
                "null"
              ]
            }
          }
        }
      },
      "additionalProperties": false
    }

    Here's an example "oneOf" item for the 'details' query, that 
    has a required calling parameter (e.g. 'id'):
    {
      "description": "Returns the details of the named swap, error if not created. Return type: DetailsResponse.",
      "type": "object",
      "required": [
        "details"
      ],
      "properties": {
        "details": {
          "type": "object",
          "required": [
            "id"
          ],
          "properties": {
            "id": {
              "type": "string"
            }
          }
        }
      },
      "additionalProperties": false
    }

* "definitions": an object containing type definitions for any custom (developer) 
defined calling parameters used by the Ambur marketplace contracts queries. This 
field is helpful because, since queries will be built as JSON messages, the 
'defnitions' object enables you to instruct your chat partner on how to provide 
the correct JSON values that satisfy the custom types defined in the Ambur 
marketplace smart contract, or that Ambur marketplace has imported as a dependency.
    - Each entry in the 'definitions' object is another object, the name of which 
    is the custom type being defined. 
    - Each of these entries contains a sub-field called "type", which details the 
    expected JSON type for the custom (developer) defined type
    - If the JSON type declared in "type" sub-field is "object", the entry will 
    contain other schema fields which explain the JSON types required for the 
    object's sub-fields, and will possess a similar structure to schema fields 
    discussed earlier in this documentation.

    Here's an example of a custom defintion (e.g. an item from the 'definitions' 
    object) showing the 'Addr' custom type expects a string value in JSON:
    "Addr": {
      "description": "A human readable address.\n\nIn Cosmos, this is typically bech32 encoded. But for multi-chain smart contracts no assumptions should be made other than being UTF-8 encoded and of reasonable length.\n\nThis type represents a validated address. It can be created in the following ways 1. Use `Addr::unchecked(input)` 2. Use `let checked: Addr = deps.api.addr_validate(input)?` 3. Use `let checked: Addr = deps.api.addr_humanize(canonical_addr)?` 4. Deserialize from JSON. This must only be done from JSON that was validated before such as a contract's state. `Addr` must not be used in messages sent by the user because this would result in unvalidated instances.\n\nThis type is immutable. If you really need to mutate it (Really? Are you sure?), create a mutable copy using `let mut mutable = Addr::to_string()` and operate on that `String` instance.",
      "type": "string"
    }

    Here's an example of a custom definition showing the 'Cw20Token' custom type 
    expects a JSON object with one property called 'address' which has an 'Addr'
    type:
    "Cw20Token": {
      "type": "object",
      "required": [
        "address"
      ],
      "properties": {
        "address": {
          "$ref": "/definitions/Addr"
        }
      }
    }
```DOCUMENTATION_END


When summarizing the schema output derived by calling this tool, always inform your 
chat partner of the names of all the available queries that can be made to the Ambur 
marketplace contract, and also inform them of the calling parameters if they ask for 
more information about a specific query."#;

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
