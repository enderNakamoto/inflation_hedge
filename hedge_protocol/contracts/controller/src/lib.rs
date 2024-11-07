#![no_std]

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BytesN, Env};

mod vault {
    soroban_sdk::contractimport!(file = "./vault.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    HedgeVault,
    RiskVault,
    TokenId,
    MaturityDate,
    Liquidated,
    Matured,
}

fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.storage().get_unchecked(key).unwrap()
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.storage().set(key, id);
}

fn get_hedge_vault(e: &Env) -> Identifier {
    e.storage().get_unchecked(&DataKey::HedgeVault).unwrap()
}

fn get_risk_vault(e: &Env) -> Identifier {
    e.storage().get_unchecked(&DataKey::RiskVault).unwrap()
}

fn get_maturity_date(e: &Env) -> u64 {
    e.storage().get_unchecked(&DataKey::MaturityDate).unwrap()
}

fn is_liquidated(e: &Env) -> bool {
    e.storage().get(&DataKey::Liquidated).unwrap_or(Ok(false)).unwrap()
}

fn is_matured(e: &Env) -> bool {
    e.storage().get(&DataKey::Matured).unwrap_or(Ok(false)).unwrap()
}

pub trait ControllerTrait {
    fn initialize(
        e: Env,
        admin: Identifier,
        hedge_vault: Identifier,
        risk_vault: Identifier,
        token_id: BytesN<32>,
        maturity_date: u64,
    );

    fn set_liquidated(e: Env);
    fn set_matured(e: Env);
    fn liquidate(e: Env);
    fn mature(e: Env);
}

pub struct ControllerContract;

#[contractimpl]
impl ControllerTrait for ControllerContract {
    fn initialize(
        e: Env,
        admin: Identifier,
        hedge_vault: Identifier,
        risk_vault: Identifier,
        token_id: BytesN<32>,
        maturity_date: u64,
    ) {
        if has_administrator(&e) {
            panic!("already initialized");
        }

        if maturity_date <= e.ledger().timestamp() {
            panic!("maturity date must be in the future");
        }

        write_administrator(&e, admin);
        e.storage().set(&DataKey::HedgeVault, hedge_vault);
        e.storage().set(&DataKey::RiskVault, risk_vault);
        e.storage().set(&DataKey::TokenId, token_id);
        e.storage().set(&DataKey::MaturityDate, maturity_date);
    }

    fn set_liquidated(e: Env) {
        let admin = read_administrator(&e);
        if admin != e.invoker().unwrap() {
            panic!("only admin can set liquidated flag");
        }

        let maturity_date = get_maturity_date(&e);
        if e.ledger().timestamp() >= maturity_date {
            panic!("cannot liquidate after maturity");
        }

        if is_matured(&e) {
            panic!("cannot liquidate after maturity");
        }

        e.storage().set(&DataKey::Liquidated, true);
    }

    fn set_matured(e: Env) {
        let admin = read_administrator(&e);
        if admin != e.invoker().unwrap() {
            panic!("only admin can set matured flag");
        }

        let maturity_date = get_maturity_date(&e);
        if e.ledger().timestamp() < maturity_date {
            panic!("cannot mature before maturity date");
        }

        if is_liquidated(&e) {
            panic!("cannot mature after liquidation");
        }

        e.storage().set(&DataKey::Matured, true);
    }

    fn liquidate(e: Env) {
        if !is_liquidated(&e) {
            panic!("not in liquidation state");
        }

        let risk_vault = get_risk_vault(&e);
        let hedge_vault = get_hedge_vault(&e);

        // Withdraw all assets from risk vault
        let risk_client = vault::Client::new(&e, &risk_vault);
        let amount = risk_client.withdraw(&hedge_vault);
    }

    fn mature(e: Env) {
        if !is_matured(&e) {
            panic!("not in matured state");
        }

        let hedge_vault = get_hedge_vault(&e);
        let risk_vault = get_risk_vault(&e);

        // Withdraw all assets from hedge vault
        let hedge_client = vault::Client::new(&e, &hedge_vault);
        let amount = hedge_client.withdraw(&risk_vault);
    }
}