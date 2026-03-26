#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String,
};

const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_TTL: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;
const PERSISTENT_TTL: u32 = 120 * DAY_IN_LEDGERS;
const PERSISTENT_THRESHOLD: u32 = 119 * DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TotalSupply,
    TotalDonated,
    DonationCount,
    Balance(Address),
    Donation(u32),
}

#[derive(Clone)]
#[contracttype]
pub struct DonationRecord {
    pub donor: Address,
    pub amount: i128,
    pub note: String,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ScholarshipError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    InvalidAmount = 4,
    DonationNotFound = 5,
}

#[contract]
pub struct ScholarshipFundContract;

#[contractimpl]
impl ScholarshipFundContract {
    pub fn __constructor(env: Env, admin: Address) -> Result<(), ScholarshipError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ScholarshipError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().set(&DataKey::TotalDonated, &0_i128);
        env.storage().instance().set(&DataKey::DonationCount, &0_u32);
        extend_instance(&env);

        env.events().publish((symbol_short!("init"),), admin);
        Ok(())
    }

    pub fn mint_scholarship(
        env: Env,
        to: Address,
        amount: i128,
    ) -> Result<(), ScholarshipError> {
        require_positive(amount)?;
        require_admin(&env)?;

        let balance_key = DataKey::Balance(to.clone());
        let balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        env.storage().persistent().set(&balance_key, &(balance + amount));
        extend_persistent(&env, &balance_key);

        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        extend_instance(&env);

        env.events()
            .publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    pub fn donate(env: Env, donor: Address, amount: i128, note: String) -> Result<u32, ScholarshipError> {
        require_positive(amount)?;
        ensure_initialized(&env)?;
        donor.require_auth();

        let donation_id: u32 = env.storage().instance().get(&DataKey::DonationCount).unwrap_or(0) + 1;
        let record = DonationRecord {
            donor: donor.clone(),
            amount,
            note,
        };
        let donation_key = DataKey::Donation(donation_id);
        env.storage().persistent().set(&donation_key, &record);
        extend_persistent(&env, &donation_key);

        let total_donated: i128 = env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalDonated, &(total_donated + amount));
        env.storage().instance().set(&DataKey::DonationCount, &donation_id);
        extend_instance(&env);

        env.events().publish(
            (symbol_short!("donate"), donor, donation_id),
            amount,
        );

        Ok(donation_id)
    }

    pub fn balance(env: Env, student: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(student))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    pub fn total_donated(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalDonated).unwrap_or(0)
    }

    pub fn donation_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::DonationCount).unwrap_or(0)
    }

    pub fn admin(env: Env) -> Result<Address, ScholarshipError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ScholarshipError::NotInitialized)
    }

    pub fn donation(env: Env, donation_id: u32) -> Result<DonationRecord, ScholarshipError> {
        let donation_key = DataKey::Donation(donation_id);
        let record = env
            .storage()
            .persistent()
            .get(&donation_key)
            .ok_or(ScholarshipError::DonationNotFound)?;
        extend_persistent(&env, &donation_key);
        Ok(record)
    }
}

fn ensure_initialized(env: &Env) -> Result<(), ScholarshipError> {
    if env.storage().instance().has(&DataKey::Admin) {
        Ok(())
    } else {
        Err(ScholarshipError::NotInitialized)
    }
}

fn require_admin(env: &Env) -> Result<(), ScholarshipError> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ScholarshipError::NotInitialized)?;
    admin.require_auth();
    Ok(())
}

fn require_positive(amount: i128) -> Result<(), ScholarshipError> {
    if amount <= 0 {
        Err(ScholarshipError::InvalidAmount)
    } else {
        Ok(())
    }
}

fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
}

fn extend_persistent(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_THRESHOLD, PERSISTENT_TTL);
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, ScholarshipFundContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(ScholarshipFundContract, (&admin,));
        let client = ScholarshipFundContractClient::new(&env, &contract_id);

        (env, client, admin)
    }

    #[test]
    fn test_mint_scholarship() {
        let (env, client, admin) = setup();
        let student = Address::generate(&env);

        assert_eq!(client.admin().unwrap(), admin);
        client.mint_scholarship(&student, &500);

        assert_eq!(client.balance(&student), 500);
        assert_eq!(client.total_supply(), 500);
    }

    #[test]
    fn test_donate_and_read_record() {
        let (env, client, _) = setup();
        let donor = Address::generate(&env);

        let donation_id = client.donate(
            &donor,
            &1_000,
            &String::from_str(&env, "Quyen gop hoc bong ky mua thu"),
        );

        let record = client.donation(&donation_id).unwrap();
        assert_eq!(donation_id, 1);
        assert_eq!(record.donor, donor);
        assert_eq!(record.amount, 1_000);
        assert_eq!(client.total_donated(), 1_000);
        assert_eq!(client.donation_count(), 1);
    }
}
