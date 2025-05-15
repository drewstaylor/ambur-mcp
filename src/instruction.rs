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
// Contracts & Collections
pub static LIST_CONTRACTS_DESCR: &str = r#"
Call this tool to get a list of contract addresses, and associated Archway Network, 
where the Ambur marketplace contract has been deployed. This tool is helpful for 
discovering the mainnet and testnet contract addresses for the Ambur marketplace 
smart contract."#;

pub static LIST_NFT_COLLECTIONS_DESCR: &str = r#"
Call this tool to get a list of NFT collections that can be traded in the Ambur marketplace 
contract. The detailed response will provide the following information about each NFT 
collection:

```DOCUMENTATION_BEGIN
* Collection name
* Collection description
* Collection contract addresses for the token contract (the nft) and the minter contract (a 
manager contract that operates as admin over the token contract)
    - Users will need to make txs to the minter contract for certain behaviours, including: 
    minting NFTs, revealing NFT metadata, and redeeming Foresight NFTs for entry into the 
    whitelist for minting newly launching collections.
```DOCUMENTATION_END

This tool is helpful for discovering the mainnet and tesnet contract addresses for tokens 
listed for trading in the Ambur marketplace smart contract."#;

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
      "description": "A human readable blockchain address. In Cosmos, this is typically bech32 encoded...",
      "type": "string"
    }

    Here's an example of a custom definition showing the 'Cw20Token' custom type 
    expects a JSON object with one property called 'address' which has an 'Addr'
    type (which is a custom definition that equates to 'string' in JSON):
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

pub static LIST_QUERY_RESPONSE_DESCR: &str = r#"
Call this tool to get a JSON schema for common query response types. This tool is 
useful when your chat partner asks for more information about a specific query, if 
you've already called the 'list_query_entry_points' tool and it has informed you 
of the return type for the query. 

The output from this tool is an object containing the JSON schema for the following 
custom (developer) defined types:

```DOCUMENTATION_BEGIN
### All Query Return Types:
* `CollectionOfferDetailsResponse`
* `CollectionRoyaltiesResponse`
* `DetailsResponse`
* `ListResponse`

**Notes:** 
* `CollectionRoyaltiesResponse` has a sub-field called `fee_percentage` that has a format 
of `uint64` in Rust and a type of `integer` in JSON representation. This 'fee_percentage' is 
a whole number with a minimum value of 0 (e.g. 0% fees) and a maximum value of 30 (e.g. 30% 
fees). `fee_percentage` is _not_ currently denominated in basis points.
```DOCUMENTATION_END
"#;

// Prepare a query message for RPC broadcast
pub static BUILD_QUERY_MSG_DESCR: &str = r#"
Call this tool to build a prepared query message for a query to the Ambur NFT marketplace 
contract. This tool won't broadcast the query or return the query result, but can be 
combined with any RPC connected query tool that accepts a well-formed Cosmos QueryRequest.

There are two calling parameters required when calling this tool: the Ambur contract address 
('contract_addr', e.g. either the mainnet or testnet contract address; see tool: 
'list_contract_deployments'), and QueryMsg variant to be built into a Cosmos QueryRequest.

See the below documentation for more info about QueryMsg variants: 

```DOCUMENTATION_BEGIN
* Here's an example of a QueryMsg variant (e.g. for the 'list_allowed_payments' query) with 
no required parameter values:

{
  "list_allowed_payments": {}
}

* Here's an example of a QueryMsg variant (e.g. for the 'details' query) with required 
parameter values:

{
  "details": {
    "id": "swap-id-52d709fd-8cfe-45e1"
  }
}

* Here's an example of a QueryMsg variant (e.g. for the 'list' query) where the optional 
parameters are ignored:

{
  "list": {
    "limit": null,
    "start_after": null
  }
}
```DOCUMENTATION_END

Submit both the `contract_addr` and `query_msg` calling parameters in string format. For 
example, a `details` QueryMsg variant can be submitted to the tool like this: 
"{\"details\":{\"id\":\"cca4e046-97ba-45b2-841b-9adca039545e\"}}"
"#;

// Execute
pub static LIST_TX_ENTRY_POINTS_DESCR: &str = r#"
Call this tool to get a list of possible transactions that can be made (e.g.  
execute entry points) to the Ambur marketplace contract, as well as their 
associated calling parameters. This tool is helpful for discovering what parameters 
a user must provide in order to build a perpared execute message for a tx to the 
Ambur NFT marketplace smart contract.

The response provided from this tool is a JSON schema for the ExecuteMsg enum of the 
Ambur maketplace contract. It would be too verbose to provide it to your chat 
partner, so summarizing it will be crucial. 

Below is some documentation to help you parse and understand the fields and 
structure of the generated ExecuteMsg schema, so that you can extrapolate the 
possible queries that can be made to the Ambur smart contract (some non-relevant 
fields are excluded from the documentation and can be safely ignored):

```DOCUMENTATION_BEGIN
* "$schema": a metadata field clarifying which version of JSON schema was used for 
generating this schema

* "title": a field that clarifies what item was serialized into this JSON schema 
(e.g. in this case the 'ExecuteMsg' enum from the Ambur marketplace smart contract)

* "oneOf": an array of all transactions (txs) that can be made to the Ambur
marketplace smart contract, as well as their calling parameters. 
    - "required": the 'required' sub-field of 'oneOf' items is an array containing 
    a single value. The single value contained by this array is the name of a 
    valid tx that can be made to the Ambur marketplace contract
    - "properties": the 'properties' sub-field of these 'oneOf' items provides an 
    object representing the tx that contains any required and/or optional calling 
    parameters. Since each calling parameter for Ambur txs are custom (developer) 
    defined types, the calling parameters (contained by the tx object of the 
    'properties' sub-field) will use a '$ref' key with a value that points to where 
    the custom type definition can be located in the 'definitions' object of the 
    provided schema for the 'ExecuteMsg' enum of the Ambur marketplace contract.

    Here's an example "oneOf" item for the 'finish' tx (which consumes and executes 
    a swap, AKA a trade, on Ambur marketplace):
    {
      "type": "object",
      "required": [
        "finish"
      ],
      "properties": {
        "finish": {
          "$ref": "/definitions/FinishSwapMsg"
        }
      },
      "additionalProperties": false
    }

* Since the generated JSON schema for Ambur's 'ExecuteMsg' enum doesn't include 
'description' sub-fields for its 'oneOf' items, here's some basic info describing 
each tx entry point (e.g. 'ExecuteMsg' variant) of Ambur marketplace contract:
    - `create`: Creates a swap of type 'Sale' or 'Offer'. 'Offer' swaps can only be 
    created with a CW20 token as the payment token (e.g. to avoid needing to hold the 
    offerer's funds in escrow)
    - `finish`: Consumes a swap of type 'Sale' or 'Offer' and processes asset 
    ownership transfers; e.g. NFT(s) transfered to the buyer, and payment tokens
    are transferred to the seller. If the payment token is native (e.g. ARCH), the buyer 
    must send the correct native funds amount in their tx (e.g. in the 'funds' array of 
    the 'CosmosMsg'). For 'Offers' the CW20 tokens will be automatically transferred from 
    buyer to the seller, assuming i) the buyer has sufficient token balance, and ii) has 
    approved the Ambur marketplace contract to spend the required payment token amount
    - `cancel`: Cancels a created swap, must be called by the swap's creator
    - `update`: Update either the price or expiration, or both, of a given swap. If 
    the swap type is 'Offer', only the expiration can be 
    updated. Must be called by the swap's creator
    - `create_collection_offer`: Creates an offer to buy 1 or more NFTs from a 
    given NFT collection. Sellers may fulfill this order with any NFT(s) from the 
    collection regardless of their 'token_id's. The `price` field of the collection offer 
    refers to the price to purchase all the NFTs being offered as a batch, it is _not_ the 
    price per NFT. Collection offers can only be created with a CW20 token as the payment 
    token (e.g. to avoid needing to hold the offerer's funds in escrow)
    - `cancel_collection_offer`: Cancel a collection offer. Must be called by the 
    swap's creator
    - `finish_collection_offer`: Consumes a collection offer swap and processes 
    asset ownership transfers; e.g. NFT(s) transfered to the buyer, and payment tokens
    are transferred to the seller
    - `update_config`: Admin only tx to update the contract's configuration 
    parameters
    - `add_nft`: Admin only tx to give permission for an NFT collection to be traded 
    in Ambur marketplace
    - `remove_nft`: Admin only tx to remove permission for an NFT collection to be
    traded in Ambur marketplace
    - `update_nft`: Admin only tx to modify royalties settings (e.g. royalty fee 
    percentage, and/or royalty recipient address) for an NFT collection in Ambur 
    marketplace. 
    - `withdraw`: Admin only tx to withdraw funds stored in the contract (e.g. 
    withdraw accrued marketplace fees)
    - `allow_payments`: Admin only tx to give permission for creating swaps with a 
    given payment token (e.g. native token, or CW20 token)
    - `disallow_payments`: Admin only tx to remove permission for creating swaps 
    with a given payment token (e.g. native token, or CW20 token)

* "definitions": an object containing type definitions for any custom (developer) 
defined calling parameters used by the Ambur marketplace contracts txs. This 
field is helpful because, since txs will be built as JSON messages, the 
'defnitions' object enables you to instruct your chat partner on how to provide 
the correct JSON values that satisfy the custom types for tx parameters as defined 
in the Ambur marketplace smart contract.
```DOCUMENTATION_END

When summarizing the schema output derived by calling this tool, always inform your 
chat partner of the names of all the available txs that can be made to the Ambur 
marketplace contract, and also inform them of the calling parameters if they ask for 
more information about a specific transaction."#;

// Prepare a transaction message for signing and RPC broadcast
pub static BUILD_EXECUTE_MSG_DESCR: &str = r#"
Call this tool to build a prepared execute message for a transaction to the Ambur 
NFT marketplace contract. This tool won't sign the message, or broadcast it to the 
blockchain, but can be combined with any RPC connected tx tool that accepts a well-
formed CosmosMsg for an ExecuteMsg variant for any valid Ambur marketplace contract 
execute (tx) entry point.

There are three calling parameters required when calling this tool: the Ambur 
contract address ('contract_addr', e.g. either the mainnet or testnet contract address; see 
tool: 'list_contract_deployments'), the amount of native funds ('payment') to send in the 
transaction, and the ExecuteMsg variant ('execute_msg') to be built into a CosmosMsg that can 
be signed and broadcast by an RPC connected signing wallet.

See the below documentation for more info about funding amounts and the ExecuteMsg variants: 

```DOCUMENTATION_BEGIN
* All 'create' swaps with swap type 'Sale' must use Archway Network's USDC native 
coin as the payment token, which has a denom value of 
'ibc/43897B9739BD63E3A08A88191999C632E052724AB96BD4C74AE31375C991F48D'

* All 'create' swaps with swap type 'Offer' must use Archway Network's wUSDC CW20 coin as 
the payment token, which has a denom of 'wUSDC' and a contract address of 
archway1gaf9nw7n8v5lpjz9caxjpps006kxfcrzcuc8y5qp4clslhven2ns2g0ule

* Both USDC and wUSDC have a decimal precision of 6 decimals (10**6)

* Here's an example of a ExecuteMsg variant (e.g. for a Sale 'create' tx) with its required 
parameter values:

{
  "create": {
    "id": "cca4e046-97ba-45b2-841b-9adca039545e",
    "cw721": "archway1r9qqfl2ptc96frn3tx4k2n967xc64uwxg2j9xn2rvsm882fu04kq3hutsv",
    "token_id": "840",
    "payment_token": {
      "native": {
        "denom": "ibc/43897B9739BD63E3A08A88191999C632E052724AB96BD4C74AE31375C991F48D"
      }
    },
    "price": "8880000",
    "swap_type": "Sale",
    "expires": {
      "never": {}
    }
  }
}

* Here's an example of a ExecuteMsg variant (e.g. for an Offer 'create' tx) with its required 
parameter values:

{
  "create": {
    "id": "9fb7be1f-0a25-451a-bc5e-31b13d9b850b",
    "cw721": "archway1r9qqfl2ptc96frn3tx4k2n967xc64uwxg2j9xn2rvsm882fu04kq3hutsv",
    "token_id": "427",
    "payment_token": {
      "cw20": {
        "address": "archway1gaf9nw7n8v5lpjz9caxjpps006kxfcrzcuc8y5qp4clslhven2ns2g0ule"
      }
    },
    "price": "888888",
    "swap_type": "Offer",
    "expires": {
      "at_time": "1747415615458000000"
    }
  }
}

* Here's the schema for adding a native payment to a transaction to be built (e.g. for 
'finish' txs with a native payment token):

{
  "denom": "ibc/43897B9739BD63E3A08A88191999C632E052724AB96BD4C74AE31375C991F48D",
  "amount": "888888",
}

**NOTE**: "ibc/43897B9739BD63E3A08A88191999C632E052724AB96BD4C74AE31375C991F48D" is the 
denom for USDC on Archway Network (which is currently the only allowed native payment 
token).
```DOCUMENTATION_END

Submit all three calling parameters (`contract_addr`, `execute_msg` and `payment`) 
to tool in string format. For example, use stringified JSON for the `execute_msg` and 
`payment` calling parameters."#;
