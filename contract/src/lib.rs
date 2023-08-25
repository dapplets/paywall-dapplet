use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, CryptoHash, Promise};
use near_sdk::{AccountId, BorshStorageKey};

type ContentId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    purchases_per_account: LookupMap<AccountId, UnorderedSet<ContentId>>,
}
impl Default for Contract {
    fn default() -> Self {
        Self {
            purchases_per_account: LookupMap::new(StorageKeys::PurchasesPerAccount),
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    PurchasesPerAccount,
    PurchasesPerAccountInner { account_hash: CryptoHash },
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn buy(&mut self, content_id: ContentId) -> Promise {
        let account_id = env::predecessor_account_id();
        let mut content_purchases =
            self.purchases_per_account
                .get(&account_id)
                .unwrap_or_else(|| {
                    UnorderedSet::new(StorageKeys::PurchasesPerAccountInner {
                        account_hash: env::sha256_array(account_id.as_bytes()),
                    })
                });

        // Check if the content has already been purchased
        assert!(
            !content_purchases.contains(&content_id),
            "Content already purchased"
        );

        // Add the content to the purchased list for the account
        content_purchases.insert(&content_id);
        self.purchases_per_account
            .insert(&account_id, &content_purchases);

        // Return attached deposit
        Promise::new(account_id.clone()).transfer(env::attached_deposit())
    }

    pub fn purchased(&self, account_id: AccountId, content_id: ContentId) -> bool {
        let content_purchases = self.purchases_per_account.get(&account_id);
        match content_purchases {
            Some(purchases) => purchases.contains(&content_id),
            None => false,
        }
    }

    pub fn purchases(&self, account_id: &AccountId) -> Vec<ContentId> {
        let tokens = self
            .purchases_per_account
            .get(account_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(StorageKeys::PurchasesPerAccountInner {
                    account_hash: env::sha256_array(account_id.as_bytes()),
                })
            });

        tokens.to_vec()
    }

    pub fn clear_my_purchases(&mut self) {
        let account_id = env::predecessor_account_id();
        let mut content_purchases =
            self.purchases_per_account
                .get(&account_id)
                .unwrap_or_else(|| {
                    UnorderedSet::new(StorageKeys::PurchasesPerAccountInner {
                        account_hash: env::sha256_array(account_id.as_bytes()),
                    })
                });

        // Add the content to the purchased list for the account
        content_purchases.clear();
        self.purchases_per_account
            .insert(&account_id, &content_purchases);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_contract() -> Contract {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(accounts(0));
        builder.predecessor_account_id(accounts(1));
        builder.attached_deposit(10_000_000_000_000_000_000_000);

        testing_env!(builder.build());

        let contract = Contract {
            purchases_per_account: LookupMap::new(StorageKeys::PurchasesPerAccount),
        };

        contract
    }

    #[test]
    fn test_buy_content() {
        let mut contract = get_contract();
        let content_id = "content123".to_string();

        // Buy the content
        contract.buy(content_id.clone());

        // Check if the content was purchased
        assert!(contract.purchased(accounts(1), content_id.clone()));
    }

    #[test]
    fn test_buy_with_attached_deposit() {
        let mut contract = get_contract();
        let content_id = "content123".to_string();

        let balance_before = env::account_balance();

        contract.buy(content_id.clone());

        // Check if the attached deposit was returned
        assert_eq!(
            env::account_balance(),
            balance_before - 10_000_000_000_000_000_000_000
        );
    }

    #[test]
    #[should_panic(expected = "Content already purchased")]
    fn test_buy_content_already_purchased() {
        let mut contract = get_contract();
        let content_id = "content123".to_string();

        // Buy the content
        contract.buy(content_id.clone());

        // Buy the same content again (should panic)
        contract.buy(content_id.clone());
    }

    #[test]
    fn test_purchases() {
        let mut contract = get_contract();
        let content_id1 = "content123".to_string();
        let content_id2 = "content456".to_string();

        // Buy some content
        contract.buy(content_id1.clone());

        // Check purchased content for the account
        let purchases = contract.purchases(&accounts(1));
        assert_eq!(purchases.len(), 1);
        assert!(purchases.contains(&content_id1));

        // Buy more content
        contract.buy(content_id2.clone());

        // Check purchased content again
        let purchases = contract.purchases(&accounts(1));
        assert_eq!(purchases.len(), 2);
        assert!(purchases.contains(&content_id1));
        assert!(purchases.contains(&content_id2));
    }

    #[test]
    fn test_not_purchased_content() {
        let contract = get_contract();
        let content_id = "content123".to_string();

        // Check if the content was not purchased
        assert!(!contract.purchased(accounts(1), content_id));
    }

    #[test]
    fn test_clear_purchases() {
        let mut contract = get_contract();
        let content_id1 = "content123".to_string();

        // Buy some content and clear purchases
        contract.buy(content_id1.clone());
        contract.clear_my_purchases();

        // Check purchased content
        let purchases = contract.purchases(&accounts(1));
        assert_eq!(purchases.len(), 0);
    }
}
