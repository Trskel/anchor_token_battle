Here is a professional and comprehensive README.md in English, specifically tailored for your GitHub repository. It explains the game mechanics, the technical architecture of Solana/Anchor, and how the token economy works.

Markdown
# Boss Golden Dragon Battle 🐉 💰

A decentralized combat and economy game built with the **Anchor Framework** on the Solana blockchain. Players manage their energy, fight a powerful Boss, and interact with a dynamic SPL token economy (**GoldenDragon Tokens**).

## 1. Project Overview

This program simulates a battle between a player and a "Golden Dragon" Boss. Beyond simple state management, it demonstrates advanced Solana concepts like **Cross-Program Invocations (CPI)**, **Associated Token Accounts (ATA)**, and **Program Derived Addresses (PDA)** to manage secure game vaults.

### Core Mechanics:
* **Initialization**: Sets up the player's game state and mints initial tokens to both the player and the Boss's treasure vault.
* **Combat (Attack)**: A turn-based system where damage is calculated using a pseudo-random generator based on the current Solana `slot`.
* **Loot Economy**:
    * **Victory**: If the player defeats the Boss (Enemy Energy = 0), they claim **100% of the Boss's treasure vault**.
    * **Defeat**: If the player is defeated (Player Energy = 0), they **lose 50% of their tokens**, which are transferred to the Boss's vault.
* **Respawn**: Resets the Boss's health and injects new tokens into the vault via a `MintTo` operation to ensure there is always loot to fight for.

---

## 2. Technical Architecture

### Account Structure
The program utilizes specialized account types to ensure security and trustless ownership:

* **GameData Account (PDA)**: Derived from the static seed `"GAME_DATA"` and the player's public key. It stores `player_energy`, `enemy_energy`, and the owner's address. It acts as the **Signing Authority** for the treasure vault.
* **GoldenDragon Mint**: The custom SPL token. The game owner acts as the Mint Authority.
* **Treasure Vault (ATA)**: A token account owned by the `GameData` PDA. No external wallet can withdraw these funds; they move strictly according to program logic.

### Technical Stack
* **Anchor Framework**: For account validation and boilerplate abstraction.
* **Solana SPL Token**: For token transfers and minting operations.
* **XORShift**: An efficient pseudo-random number generator for on-chain damage calculation.

---

## 3. Instruction Handlers

| Instruction | Description | Security Detail |
| :--- | :--- | :--- |
| `initialize` | Sets up the game state and ATAs. | Uses `init_if_needed` to handle existing token accounts gracefully. |
| `attack` | Executes combat and loot transfers. | Uses `CpiContext::new_with_signer` for PDA-authorized loot distribution. |
| `respawn` | Resets the Boss and refills the vault. | Requires the `owner` signature to authorize new token minting. |

---

## 4. Getting Started

### Prerequisites
* [Rust installed](https://www.rust-lang.org/tools/install)
* [Solana CLI installed](https://docs.solanalabs.com/cli/install)
* [Anchor CLI installed](https://www.anchor-lang.com/docs/installation) (v0.30.1 recommended)

### Setup & Installation
1.  Clone the repository:
    ```bash
    git clone [https://github.com/Trskel/anchor_token_battle.git](https://github.com/Trskel/anchor_token_battle.git)
    cd anchor_token_battle
    ```
2.  Install dependencies:
    ```bash
    yarn install
    ```
3.  Build the program:
    ```bash
    anchor build
    ```
4.  Run tests:
    ```bash
    anchor test
    ```

---

## 5. Project Structure

* `programs/boss_battle/src/instructions/`: Modularized logic for `initialize.rs`, `attack.rs`, and `respawn.rs`.
* `programs/boss_battle/src/state/`: Data structures for the `GameData` account.
* `programs/boss_battle/src/utils/`: Helper functions (e.g., Randomness generator).
* `programs/boss_battle/src/constants/`: Game balance configurations (Max energy, damage, seeds).

---

## 6. Security & Constraints
The program implements rigorous security checks:
* **Ownership Validation**: `constraint = game_data_account.owner == owner.key()` ensures only the legitimate player can trigger their own battle.
* **Deterministic PDAs**: Accounts are strictly bound to the player's public key, preventing account collision or unauthorized state access.

---

**Developed by [Your Name/Handle]** – Exploring secure CPIs and PDA-owned vaults on S
