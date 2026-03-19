#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Bet {
    pub user: Address,
    pub amount: i128,
    pub prediction: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub struct Game {
    pub team1: Symbol,
    pub team2: Symbol,
    pub result: Symbol,
    pub resolved: bool,
}

#[contract]
pub struct SportsBettingPlatform;

#[contractimpl]
impl SportsBettingPlatform {

    // Create a new game
    pub fn create_game(env: Env, game_id: Symbol, team1: Symbol, team2: Symbol) {
        let game = Game {
            team1,
            team2,
            result: Symbol::new(&env, "PENDING"),
            resolved: false,
        };

        env.storage().instance().set(&game_id, &game);
    }

    // Place a bet
    pub fn place_bet(
        env: Env,
        game_id: Symbol,
        user: Address,
        prediction: Symbol,
        amount: i128,
    ) {
        user.require_auth();

        let key = (game_id.clone(), Symbol::new(&env, "bets"));

        let mut bets: Map<Address, Bet> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let bet = Bet {
            user: user.clone(),
            amount,
            prediction,
        };

        bets.set(user.clone(), bet);

        env.storage().instance().set(&key, &bets);
    }

    // Resolve game
    pub fn resolve_game(env: Env, game_id: Symbol, result: Symbol) {
        let mut game: Game = env.storage().instance().get(&game_id).unwrap();

        game.result = result;
        game.resolved = true;

        env.storage().instance().set(&game_id, &game);
    }

    // Claim winnings
    pub fn claim(env: Env, game_id: Symbol, user: Address) -> i128 {
        user.require_auth();

        let game: Game = env.storage().instance().get(&game_id).unwrap();
        if !game.resolved {
            panic!("Game not resolved");
        }

        let key = (game_id, Symbol::new(&env, "bets"));

        let bets: Map<Address, Bet> =
            env.storage().instance().get(&key).unwrap();

        let bet = bets.get(user.clone()).unwrap();

        if bet.prediction != game.result {
            return 0;
        }

        // Simple payout: 2x
        bet.amount * 2
    }
}