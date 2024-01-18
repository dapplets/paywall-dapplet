use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, CryptoHash, Promise};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type ContextId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PaidContent {
    id: String,
    link: String,
    author: AccountId,
    cost: String,
    context_id: ContextId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ReturningContent {
    content: PaidContent,
    is_purchased: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    purchases_per_account: LookupMap<AccountId, UnorderedSet<String>>,
    paid_content_per_context: LookupMap<ContextId, UnorderedSet<String>>,
    paid_contents: LookupMap<String, PaidContent>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            purchases_per_account: LookupMap::new(StorageKeys::PurchasesPerAccount),
            paid_content_per_context: LookupMap::new(StorageKeys::PaidContentPerContext),
            paid_contents: LookupMap::new(StorageKeys::PaidContents),
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    PurchasesPerAccount,
    PurchasesPerAccountInner { account_hash: CryptoHash },
    PaidContentPerContext,
    PaidContentPerContextInner { account_hash: CryptoHash },
    PaidContents,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[near_bindgen]
impl Contract {
    pub fn add_paid_content(&mut self, link: String, cost: String, context_id: String) -> String {
        let author = env::predecessor_account_id();
        let id = calculate_hash(&(context_id.clone() + &link)).to_string();

        // Check if the content with the same I has already been added
        assert!(
            !self.paid_contents.contains_key(&id),
            "Content has already been added"
        );

        // Create a PaidContent struct
        let paid_content = PaidContent {
            id: id.clone(),
            link: link.to_string(),
            author: author.clone(),
            cost,
            context_id: context_id.clone(),
        };

        // Add the paid content to the storage
        self.paid_contents.insert(&id, &paid_content);

        // Add the id to the context
        let mut context_contents = self
            .paid_content_per_context
            .get(&context_id.clone())
            .unwrap_or_else(|| {
                UnorderedSet::new(StorageKeys::PaidContentPerContextInner {
                    account_hash: env::sha256_array(context_id.as_bytes()),
                })
            });

        context_contents.insert(&id.clone());

        self.paid_content_per_context
            .insert(&context_id, &context_contents);

        // Add the content to the purchased list of the author
        let mut content_purchases = self.purchases_per_account.get(&author).unwrap_or_else(|| {
            UnorderedSet::new(StorageKeys::PurchasesPerAccountInner {
                account_hash: env::sha256_array(author.as_bytes()),
            })
        });
        content_purchases.insert(&id.clone());
        self.purchases_per_account
            .insert(&author, &content_purchases);

        id
    }

    pub fn get_content_by_post_for_account(
        &self,
        context_id: ContextId,
        account_id: AccountId,
    ) -> Vec<ReturningContent> {
        assert!(
            self.paid_content_per_context.contains_key(&context_id),
            "There is no content with this ID"
        );
        let ids = self
            .paid_content_per_context
            .get(&context_id.to_string())
            .unwrap()
            .to_vec();
        let mut returned_contents = Vec::new();
        for id in ids {
            let content = self.paid_contents.get(&id).unwrap();
            let is_purchased = self.is_purchased(&account_id, id);
            let returned_content = ReturningContent {
                content,
                is_purchased,
            };
            returned_contents.push(returned_content);
        }
        returned_contents
    }

    #[payable]
    pub fn buy(&mut self, content_id: String) -> Promise {
        let id = content_id.to_string();

        // Check if the content exists
        assert!(
            self.paid_contents.contains_key(&id),
            "No content found with this ID"
        );

        let account_id = env::predecessor_account_id();

        // Check if the buyer in the author of the content
        assert_ne!(
            self.paid_contents.get(&id).unwrap().author,
            account_id,
            "The buyer and the author are the same account"
        );

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
            !content_purchases.contains(&id),
            "Content already purchased"
        );

        let content = self.paid_contents.get(&id).unwrap();
        let attached = env::attached_deposit().to_string();

        // Check if the attached_deposit is wrong
        assert!(
            content.cost == attached,
            "Attached deposit is wrong. It should be {} instead of {}",
            content.cost,
            attached
        );

        // Add the content to the purchased list for the account
        content_purchases.insert(&id);
        self.purchases_per_account
            .insert(&account_id, &content_purchases);

        // Transfer attached deposit to the author
        Promise::new(content.author).transfer(env::attached_deposit())
    }

    pub fn is_purchased(&self, account_id: &AccountId, content_id: String) -> bool {
        let content_purchases = self.purchases_per_account.get(&account_id);
        match content_purchases {
            Some(purchases) => purchases.contains(&content_id.to_string()),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, Balance};

    const NEAR: u128 = 1_000_000_000_000_000_000_000_000;

    fn create_context(predecessor: String, deposit: Balance) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(deposit);
        builder
    }

    // Test adding paid content
    #[test]
    fn test_add_paid_content() {
        let mut contract = Contract::default();
        let bob_id = "bob.near";
        let builder = create_context(bob_id.to_string(), 0);
        testing_env!(builder.build());

        let link = "https://example.com/content";
        let cost = 2 * NEAR;
        let context_id = "context123".to_string();

        // Add paid content
        let received_id =
            contract.add_paid_content(link.to_string(), cost.to_string(), context_id.clone());

        // Check if content has been added
        let content_id = calculate_hash(&format!("{context_id}{link}")).to_string();
        assert_eq!(content_id, received_id);
        assert!(contract.paid_contents.contains_key(&content_id));

        // Check if context contains content
        let context_contents = contract.paid_content_per_context.get(&context_id).unwrap();
        assert!(context_contents.contains(&content_id));
    }

    // Test getting content for an account
    #[test]
    fn test_get_content_by_post_for_account() {
        let mut contract = Contract::default();
        let bob_id = "bob.near";
        let builder = create_context(bob_id.to_string(), 0);
        testing_env!(builder.build());

        // Add paid content
        let link = "https://example.com/content";
        let cost = 2 * NEAR;
        let context_id = "context123".to_string();
        contract.add_paid_content(link.to_string(), cost.to_string(), context_id.clone());

        // Get content for an account (view function)
        let account_id: AccountId = bob_id.parse().unwrap();
        let result = contract.get_content_by_post_for_account(context_id, account_id);
        let returned_content = &result[0];
        assert_eq!(returned_content.is_purchased, true);
        assert_eq!(returned_content.content.link, link);
        assert_eq!(returned_content.content.cost, cost.to_string());
    }

    // Test buying content
    #[test]
    fn test_buy_content() {
        let mut contract = Contract::default();
        let bob_id = "bob.near";
        let bob_builder = create_context(bob_id.to_string(), 0);
        let bob_context = bob_builder.build();
        testing_env!(bob_context.clone());

        // Add paid content
        let link = "https://example.com/content";
        let cost = 2 * NEAR;
        let context_id = "context123".to_string();
        let content_id = calculate_hash(&format!("{context_id}{link}")).to_string();
        contract.add_paid_content(link.to_string(), cost.to_string(), context_id.clone());

        // Create another account and buy the content
        let alexa_id = "alexa.near";
        let alexa_builder = create_context(alexa_id.to_string(), 2 * NEAR);
        let alexa_context = alexa_builder.build();
        testing_env!(alexa_context);
        let alexa_balance_before = env::account_balance();

        // Check if content has not been marked as purchased for the account yet
        assert!(!contract.is_purchased(&alexa_id.parse().unwrap(), content_id.clone()));

        // Purchase content
        contract.buy(content_id.clone());

        // Check if the attached deposit was removed
        assert_eq!(env::account_balance(), alexa_balance_before - 2 * NEAR);

        // Check if content is marked as purchased for the account
        assert!(contract.is_purchased(&alexa_id.parse().unwrap(), content_id.clone()));

        // Check the author receive tokens
        testing_env!(bob_context);
        assert!(contract.is_purchased(&bob_id.parse().unwrap(), content_id.clone()));
    }

    #[test]
    #[should_panic(expected = "The buyer and the author are the same account")]
    fn test_author_buy_his_own_content() {
        let mut contract = Contract::default();
        let account_id = "bob.near".to_string();
        let builder = create_context(account_id, 2 * NEAR);
        testing_env!(builder.build());

        // Add paid content
        let link = "https://example.com/content";
        let cost = 2 * NEAR;
        let context_id = "context123".to_string();
        let content_id = calculate_hash(&format!("{context_id}{link}")).to_string();
        contract.add_paid_content(link.to_string(), cost.to_string(), context_id.clone());

        // Purchase content
        contract.buy(content_id);
    }

    #[test]
    #[should_panic(
        expected = "Attached deposit is wrong. It should be 2000000000000000000000000 instead of 0"
    )]
    fn test_buy_content_no_money() {
        let mut contract = Contract::default();
        let bob_id = "bob.near".to_string();
        let bob_builder = create_context(bob_id, 0);
        let bob_context = bob_builder.build();
        testing_env!(bob_context.clone());

        // Add paid content
        let link = "https://example.com/content";
        let cost = 2 * NEAR;
        let context_id = "context123".to_string();
        let content_id = calculate_hash(&format!("{context_id}{link}")).to_string();
        contract.add_paid_content(link.to_string(), cost.to_string(), context_id.clone());

        // Create another account and buy the content
        let alexa_id = "alexa.near";
        let alexa_builder = create_context(alexa_id.to_string(), 0);
        let alexa_context = alexa_builder.build();
        testing_env!(alexa_context);

        // Check if content has not been marked as purchased for the account yet
        assert!(!contract.is_purchased(&alexa_id.parse().unwrap(), content_id.clone()));

        // Purchase content
        contract.buy(content_id.clone());
    }
}
