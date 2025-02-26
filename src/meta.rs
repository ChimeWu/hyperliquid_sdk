use std::collections::HashMap;

use chrono::Timelike;
use ethers::abi::ethereum_types::H128;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub universe: Vec<AssetMeta>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetMeta {
    pub name: String,
    pub sz_decimals: u32,
    pub max_leverage: Option<u32>,
    pub is_delisted: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetContext {
    pub funding: String,
    pub open_interest: String,
    pub prev_day_px: String,
    pub day_ntl_vlm: String,
    pub premium: Option<String>,
    pub oracle_px: String,
    pub mark_px: String,
    pub mid_px: Option<String>,
    pub impact_pxs: Option<[String; 2]>,
    pub day_base_vlm: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MetaAndAssetCtxs {
    Meta(Meta),
    Context(Vec<AssetContext>),
}

#[derive(Debug, Clone)]
pub struct FundingRate {
    pub coin: String,
    pub mark_price: String,
    pub index_price: String,
    pub rate: String,
    pub rate_estimate: String,
    pub interval: u64,
    pub next_apply_ts: u64,
    pub ts: u64,
}

impl FundingRate {
    pub fn construct(
        meta: Vec<AssetMeta>,
        context: Vec<AssetContext>,
        predict_fundings: HashMap<String, String>,
    ) -> Vec<FundingRate> {
        let mut funding_rates = Vec::new();
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let next_ts = chrono::Utc::now()
            .date_naive()
            .and_hms_opt(chrono::Utc::now().hour() + 1, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        for (i, asset) in meta.into_iter().enumerate() {
            let ctx = context[i].clone();
            let coin = &asset.name;
            let rate_estimate = predict_fundings.get(coin).cloned();
            let funding_rate = FundingRate {
                coin: asset.name,
                mark_price: ctx.mark_px,
                index_price: ctx.oracle_px,
                rate_estimate: rate_estimate.unwrap_or(ctx.funding.clone()),
                rate: ctx.funding,
                interval: 60,
                next_apply_ts: next_ts,
                ts: now,
            };
            funding_rates.push(funding_rate);
        }
        funding_rates
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpotMeta {
    pub universe: Vec<SpotAssetMeta>,
    pub tokens: Vec<TokenInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetMeta {
    pub tokens: [usize; 2],
    pub name: String,
    pub index: usize,
    pub is_canonical: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub name: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
    pub index: usize,
    pub token_id: H128,
    pub is_canonical: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetContext {
    pub day_ntl_vlm: String,
    pub mark_px: String,
    pub mid_px: Option<String>,
    pub prev_day_px: String,
    pub circulating_supply: String,
    pub coin: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SpotMetaAndAssetCtxs {
    SpotMeta(SpotMeta),
    Context(Vec<SpotAssetContext>),
}

impl SpotMeta {
    pub fn add_pair_and_name_to_index_map(
        &self,
        mut coin_to_asset: HashMap<String, u32>,
    ) -> HashMap<String, u32> {
        let index_to_name: HashMap<usize, &str> = self
            .tokens
            .iter()
            .map(|info| (info.index, info.name.as_str()))
            .collect();

        for asset in self.universe.iter() {
            let spot_ind: u32 = 10000 + asset.index as u32;
            let name_to_ind = (asset.name.clone(), spot_ind);

            let Some(token_1_name) = index_to_name.get(&asset.tokens[0]) else {
                continue;
            };

            let Some(token_2_name) = index_to_name.get(&asset.tokens[1]) else {
                continue;
            };

            coin_to_asset.insert(format!("{}/{}", token_1_name, token_2_name), spot_ind);
            coin_to_asset.insert(name_to_ind.0, name_to_ind.1);
        }

        coin_to_asset
    }
}
