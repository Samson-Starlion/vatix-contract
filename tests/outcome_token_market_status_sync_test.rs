//! Issue #1031: keeps the outcome-token contract's local `MarketStatus`
//! mirror in variant-for-variant sync with the market contract's own
//! `MarketStatus`.
//!
//! The outcome-token crate cannot depend on the market crate directly (see
//! the doc comment on `vatix_outcome_token_contract::types::MarketStatus`),
//! so it keeps its own mirror enum for decoding the market contract's
//! `get_market_status` cross-contract call. This test maps each mirror
//! variant to the market variant it is meant to mirror using *exhaustive*
//! matches on both enums: if a variant is ever added to or removed from
//! either enum without a matching update here, this file fails to compile
//! (a non-exhaustive match error) rather than silently drifting at runtime.

use vatix_market_contract::types::MarketStatus as MarketContractStatus;
use vatix_outcome_token_contract::types::MarketStatus as OutcomeTokenStatus;

/// Maps an outcome-token mirror variant to its name. Exhaustive: adding a
/// variant to the mirror without updating this match is a compile error.
fn outcome_variant_name(status: &OutcomeTokenStatus) -> &'static str {
    match status {
        OutcomeTokenStatus::Active => "Active",
        OutcomeTokenStatus::Resolved => "Resolved",
        OutcomeTokenStatus::Canceled => "Canceled",
    }
}

/// Maps a market contract `MarketStatus` variant to its name. Exhaustive:
/// adding a variant to the market contract without updating this match is a
/// compile error — exactly the drift this test exists to catch.
fn market_variant_name(status: &MarketContractStatus) -> &'static str {
    match status {
        MarketContractStatus::Active => "Active",
        MarketContractStatus::Resolved => "Resolved",
        MarketContractStatus::Canceled => "Canceled",
    }
}

#[test]
fn outcome_token_market_status_mirror_stays_in_sync_with_market_contract() {
    let outcome_names = [
        outcome_variant_name(&OutcomeTokenStatus::Active),
        outcome_variant_name(&OutcomeTokenStatus::Resolved),
        outcome_variant_name(&OutcomeTokenStatus::Canceled),
    ];
    let market_names = [
        market_variant_name(&MarketContractStatus::Active),
        market_variant_name(&MarketContractStatus::Resolved),
        market_variant_name(&MarketContractStatus::Canceled),
    ];

    // Same count and same variant-for-variant names, in the same order, so
    // a `MarketStatus` encoded by the market contract decodes correctly
    // through the outcome-token mirror (and vice versa).
    assert_eq!(outcome_names.len(), market_names.len());
    assert_eq!(outcome_names, market_names);
}
