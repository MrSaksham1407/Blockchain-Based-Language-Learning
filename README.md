# Blockchain-Based Language Learning

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Blockchain-Based Language Learning**

## Project Description

A decentralized vocabulary tracker that helps language learners store and monitor the words they learn, along with their translations, securely on the blockchain.

## Project Vision

To empower learners globally by providing an immutable, transparent, and personal record of vocabulary progress — helping them stay consistent and accountable.

## Key Features

- ✏️ **Add Vocabulary**: Learners can add new words along with translations.
- 🔍 **Retrieve Word List**: All logged vocabulary can be viewed at any time.
- ⏳ **Track Progress Over Time**: Each word is timestamped.
- 🔒 **Secure & Decentralized**: Vocabulary is stored immutably on-chain.

## Contract Details
### Contract Address: CCZQJZSBVXWXYWQG7SQ676IEYKB65ZZ3R7HUOM3U36SD22PYBDHSAXN6

### 1. `add_word(user, word, translation)`
- Logs a vocabulary word with its translation and a timestamp.
- Requires user authentication.

### 2. `get_words(user) -> Vec<WordEntry>`
- Retrieves all vocabulary entries for the given user.

Each `WordEntry` includes:
- `word`: The word in the foreign language.
- `translation`: Its meaning in the user's native language.
- `added_at`: Timestamp of when the word was added.

---

**Never forget a word again.**  
Practice. Track. Improve — all on-chain. 🌍📚  
Built with [Soroban](https://soroban.stellar.org).
