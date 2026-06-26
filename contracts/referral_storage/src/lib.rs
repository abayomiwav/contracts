//! Referral storage — on-chain referral code registry and tier management.
//! Mirrors GMX's ReferralStorage.sol.
#![no_std]
#![allow(dependency_on_unit_never_type_fallback)]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, panic_with_error,
    Address, BytesN, Env,
};

// ─── TTL constants (#297) ─────────────────────────────────────────────────────
// Referral codes and trader links are long-lived; bump only when TTL < 15 days.
// At 5 s/ledger: PERSISTENT_BUMP_TARGET ≈ 30 days, MIN_BUMP_THRESHOLD ≈ 15 days.
const PERSISTENT_BUMP_TARGET: u32 = 518_400;
const MIN_BUMP_THRESHOLD: u32 = 259_200;

// ─── Storage key types ────────────────────────────────────────────────────────

#[contracttype]
pub enum ReferralKey {
    CodeOwner(BytesN<32>),
    TraderCode(Address),
    ReferrerTier(Address),
    TierConfig(u32),
}

#[contracttype]
enum InstanceKey {
    Initialized,
    Admin,
}

// ─── Config per tier ──────────────────────────────────────────────────────────

#[contracttype]
pub struct TierConfig {
    pub total_rebate_bps: u32,    // basis points of position fee paid back to referrer
    pub discount_share_bps: u32, // portion of that rebate forwarded to trader as discount
}

// ─── Events ───────────────────────────────────────────────────────────────────

#[contractevent(topics = ["ref_reg"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeRegistered {
    pub caller: Address,
    pub code:   BytesN<32>,
}

#[contractevent(topics = ["ref_set"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraderCodeSet {
    pub trader: Address,
    pub code:   BytesN<32>,
}

// ─── Errors ───────────────────────────────────────────────────────────────────

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Unauthorized       = 2,
    CodeAlreadyTaken   = 3,
    CodeNotFound       = 4,
    InvalidTier        = 5,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct ReferralStorage;

#[contractimpl]
impl ReferralStorage {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        if env.storage().instance().has(&InstanceKey::Initialized) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        env.storage().instance().set(&InstanceKey::Initialized, &true);
        env.storage().instance().set(&InstanceKey::Admin, &admin);
    }

    /// Register a new referral code; caller becomes the owner.
    pub fn register_code(env: Env, caller: Address, code: BytesN<32>) {
        caller.require_auth();
        let key = ReferralKey::CodeOwner(code.clone());
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, Error::CodeAlreadyTaken);
        }
        env.storage().persistent().set(&key, &caller);
        env.storage().persistent().extend_ttl(&key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        env.events().publish_event(&CodeRegistered { caller, code });
    }

    /// Set the referral code for a trader (links them to a referrer).
    pub fn set_trader_referral_code(env: Env, trader: Address, code: BytesN<32>) {
        trader.require_auth();
        // Validate code exists
        if !env.storage().persistent().has(&ReferralKey::CodeOwner(code.clone())) {
            panic_with_error!(&env, Error::CodeNotFound);
        }
        let trader_key = ReferralKey::TraderCode(trader.clone());
        env.storage().persistent().set(&trader_key, &code);
        env.storage().persistent().extend_ttl(&trader_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        // Also keep the code-owner entry alive while a trader references it.
        let owner_key = ReferralKey::CodeOwner(code.clone());
        env.storage().persistent().extend_ttl(&owner_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        env.events().publish_event(&TraderCodeSet { trader, code });
    }

    /// Look up the referral code for a trader, and return the referrer's address.
    pub fn get_trader_referrer(env: Env, trader: Address) -> Option<Address> {
        let trader_key = ReferralKey::TraderCode(trader);
        let code: BytesN<32> = env.storage().persistent().get(&trader_key)?;
        env.storage().persistent().extend_ttl(&trader_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        let owner_key = ReferralKey::CodeOwner(code);
        let referrer: Address = env.storage().persistent().get(&owner_key)?;
        env.storage().persistent().extend_ttl(&owner_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        Some(referrer)
    }

    /// Return the referral code for a trader, or None.
    pub fn get_trader_referral_code(env: Env, trader: Address) -> Option<BytesN<32>> {
        env.storage().persistent().get(&ReferralKey::TraderCode(trader))
    }

    /// Set the tier for a referrer (admin only).
    pub fn set_referrer_tier(env: Env, admin: Address, referrer: Address, tier: u32) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&InstanceKey::Admin).unwrap();
        if admin != stored_admin {
            panic_with_error!(&env, Error::Unauthorized);
        }
        if tier > 2 {
            panic_with_error!(&env, Error::InvalidTier);
        }
        let tier_key = ReferralKey::ReferrerTier(referrer);
        env.storage().persistent().set(&tier_key, &tier);
        env.storage().persistent().extend_ttl(&tier_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
    }

    /// Configure the rebate/discount parameters for a tier (admin only).
    pub fn set_tier_config(env: Env, admin: Address, tier: u32, config: TierConfig) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&InstanceKey::Admin).unwrap();
        if admin != stored_admin {
            panic_with_error!(&env, Error::Unauthorized);
        }
        if tier > 2 {
            panic_with_error!(&env, Error::InvalidTier);
        }
        let tier_key = ReferralKey::TierConfig(tier);
        env.storage().persistent().set(&tier_key, &config);
        env.storage().persistent().extend_ttl(&tier_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
    }

    /// Return the fee discount bps for a trader given their referral code, or 0 if none.
    pub fn get_trader_discount_bps(env: Env, trader: Address) -> u32 {
        let trader_key = ReferralKey::TraderCode(trader);
        let code: BytesN<32> = match env.storage().persistent().get(&trader_key) {
            Some(c) => c,
            None => return 0,
        };
        env.storage().persistent().extend_ttl(&trader_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);

        let owner_key = ReferralKey::CodeOwner(code);
        let referrer: Address = match env.storage().persistent().get(&owner_key) {
            Some(r) => r,
            None => return 0,
        };
        env.storage().persistent().extend_ttl(&owner_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);

        let tier_key = ReferralKey::ReferrerTier(referrer);
        let tier: u32 = env.storage().persistent().get(&tier_key).unwrap_or(0);
        if tier > 0 {
            env.storage().persistent().extend_ttl(&tier_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);
        }

        let config_key = ReferralKey::TierConfig(tier);
        let config: TierConfig = match env.storage().persistent().get(&config_key) {
            Some(c) => c,
            None => return 0,
        };
        env.storage().persistent().extend_ttl(&config_key, MIN_BUMP_THRESHOLD, PERSISTENT_BUMP_TARGET);

        // discount = total_rebate * discount_share / 10_000
        config.total_rebate_bps * config.discount_share_bps / 10_000
    }
}
