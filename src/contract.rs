use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::network::ArchwayNetwork;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburContract {
    pub network: ArchwayNetwork,
    pub contract_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionContract {
    pub token: [AmburContract; 2],
    pub minter: [AmburContract; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburCollection {
    pub name: String,
    pub description: String,
    pub contract_addresses: CollectionContract,
}

// Contract addresses:
// Ambur (core)
pub static CONTRACT_MAINNET: &str =
    "archway13s2wjcgx4pxwq3kw66669jt8l4jm840hhhxk7ktpj7r98wc5hvnqraj6w3";
pub static CONTRACT_CONSTANTINE: &str =
    "archway1y9twsp7sf4ae8jl5huhduv0lwxa0l4g4n9lyvnvjkp648udpwh0q9vycz4";

// Collections (tokens)
pub static TOKEN_ARCHIES_MAINNET: &str =
    "archway1r9qqfl2ptc96frn3tx4k2n967xc64uwxg2j9xn2rvsm882fu04kq3hutsv";
pub static TOKEN_ARCHIES_CONSTANTINE: &str =
    "archway1pxvqacv8d6087ajd029p08dm228tzvyf8wmrkth82sltpx6zqrnsd4ypd6";
pub static TOKEN_FORESIGHT_MAINNET: &str =
    "archway1ggr22k20yfdup43mfk2gm3y24e2a0c7ey6fzrlaxugu3uyx6a0es9hs6zm";
pub static TOKEN_FORESIGHT_CONSTANTINE: &str =
    "archway1mp2lv2kxuvl4m7sq3t7xx6xnv0duwfa2jn08lsyh29xyq6gakz5q3axzdc";
pub static TOKEN_DERPIES_MAINNET: &str =
    "archway1tq5un2mjxdz5pv0c4vtfkml7wvlma4mnvxlwedrect4w9q4wff3qvqlsha";
pub static TOKEN_DERPIES_CONSTANTINE: &str =
    "archway1gv03hf28yapyvrpwplcf7j20w3jh4pmrf7aphswhwgvsl86dp6aq4al5rk";
pub static TOKEN_GHOULS_MAINNET: &str =
    "archway16cv0gsnmdhptf6jt3llt5h7jkg5gd4gzh4vw569luc86yt0et6cssdfgzp";
pub static TOKEN_GHOULS_CONSTANTINE: &str =
    "archway16cv0gsnmdhptf6jt3llt5h7jkg5gd4gzh4vw569luc86yt0et6cssdfgzp";

// Collections (minters)
pub static MINTER_ARCHIES_MAINNET: &str =
    "archway1kdwvrsptuucyhu2jtsa3qzxpctjruek6r6jffj6cqm7y83ng575sm95czz";
pub static MINTER_ARCHIES_CONSTANTINE: &str =
    "archway1r5tc3kmrscdwuzttk7rtcxslnkcgpjdn2ux979dmsz7js4k0qvase4udvl";
pub static MINTER_FORESIGHT_MAINNET: &str =
    "archway1fr97wtrwtjts8fmrvdx4m0akmuf5zx7lp60cnwk9ssqcqaukfedsevgwqp";
pub static MINTER_FORESIGHT_CONSTANTINE: &str =
    "archway17ygl8wtlsjky5kpfplklhdhmctpdr9c32c2u8rstav0cm74ewunsdv8mzj";
pub static MINTER_DERPIES_MAINNET: &str =
    "archway1jr87nadwqmzn32dfcr62rgydsp4lucpx7xquwsfq350ztfwalcwsgcsdax";
pub static MINTER_DERPIES_CONSTANTINE: &str =
    "archway1yxqetljzjetz3egmlqtv7d997yt9lshp72ce4450f0dc5lkpzvmsyrtdrf";
pub static MINTER_GHOULS_MAINNET: &str =
    "archway1ydtf4plr46ly2w84ppq7krfsfzkgvnqj7udvuuj95797hcgvqnsqstyy0s";
pub static MINTER_GHOULS_CONSTANTINE: &str =
    "archway17grtrpcg3pgm6mvcg4g603fq8065mk3gdwng8a9kum4p8hx8jptsa9zrhn";

// Collections (descriptions)
pub static ARCHIES_DESCR: &str = "Archies is a collective that brings together Archway pioneers: innovators, creators, and those daring to push the boundaries of blockchain in unconventional ways.";
pub static FORESIGHT_DESCR: &str = "The Foresight Ticket grants you a whitelist mint spot in one of the future Ambur collections. So keep an eye out, or three, for upcoming collections. Choose wisely.";
pub static DERPIES_DESCR: &str = "A community that knows web3 can be overwhelming for both newcomers and crypto OGs. Derpies aren't afraid to ask the \"dumb\" questions or call out the bullshit, making web3 super derping simple!";
pub static GHOULS_DESCR: &str = "Ghouls are on a mission to save humanity from AI!";
