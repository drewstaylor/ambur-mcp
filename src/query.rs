use philabs_cw721_marketplace::msg::{
    CollectionOfferDetailsResponse, CollectionRoyaltiesResponse, DetailsResponse, ListResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllResponse {
    pub list_response: ListResponse,
    pub details_response: DetailsResponse,
    pub collection_royalties_response: CollectionRoyaltiesResponse,
    pub collection_offer_details_response: CollectionOfferDetailsResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidatedQuery {
    pub query_msg: String,
    pub query_request: String,
}
