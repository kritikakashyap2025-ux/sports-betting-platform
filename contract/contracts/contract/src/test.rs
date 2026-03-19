#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Symbol, Address};

#[test]
fn test_betting_flow() {
    let env = Env::default();

    // Generate users
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let contract_id = env.register_contract(None, SportsBettingPlatform);
    let client = SportsBettingPlatformClient::new(&env, &contract_id);

    let game_id = Symbol::new(&env, "GAME1");

    // Create game
    client.create_game(
        &game_id,
        &Symbol::new(&env, "TEAM_A"),
        &Symbol::new(&env, "TEAM_B"),
    );

    // Place bets
    client.place_bet(
        &game_id,
        &user1,
        &Symbol::new(&env, "TEAM_A"),
        &100,
    );

    client.place_bet(
        &game_id,
        &user2,
        &Symbol::new(&env, "TEAM_B"),
        &100,
    );

    // Resolve game (TEAM_A wins)
    client.resolve_game(&game_id, &Symbol::new(&env, "TEAM_A"));

    // Claim winnings
    let payout1 = client.claim(&game_id, &user1);
    let payout2 = client.claim(&game_id, &user2);

    assert_eq!(payout1, 200); // winner
    assert_eq!(payout2, 0);   // loser
}