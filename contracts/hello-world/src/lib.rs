#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};

#[contracttype]
pub struct WordEntry {
    pub word: String,
    pub translation: String,
    pub added_at: u64,
}

#[contracttype]
pub enum VocabKey {
    Words(Address),
}

#[contract]
pub struct LanguageLearning;

#[contractimpl]
impl LanguageLearning {
    // Add a new vocabulary word with its translation
    pub fn add_word(env: Env, user: Address, word: String, translation: String) {
        user.require_auth();

        let key = VocabKey::Words(user.clone());
        let mut vocab: Vec<WordEntry> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        vocab.push_back(WordEntry {
            word,
            translation,
            added_at: env.ledger().timestamp(),
        });

        env.storage().persistent().set(&key, &vocab);

        // log!(&env, "Word added for user {}: {}", user, word);
    }

    // Retrieve all vocabulary entries for a user
    pub fn get_words(env: Env, user: Address) -> Vec<WordEntry> {
        let key = VocabKey::Words(user);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }
}
