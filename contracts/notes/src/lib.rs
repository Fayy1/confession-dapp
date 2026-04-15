#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec
};

// Struktur data message
#[contracttype]
#[derive(Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub likes: u64,
}

// Storage key
const MESSAGE_DATA: Symbol = symbol_short!("MSGS");

#[contract]
pub struct ConfessionContract;

#[contractimpl]
impl ConfessionContract {

    // 📖 Get all messages
    pub fn get_messages(env: Env) -> Vec<Message> {
        env.storage()
            .instance()
            .get(&MESSAGE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ➕ Post anonymous message
    pub fn post_message(env: Env, content: String) -> String {
        // 1. ambil data
        let mut messages: Vec<Message> = env.storage()
            .instance()
            .get(&MESSAGE_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. buat message baru
        let msg = Message {
            id: env.prng().gen::<u64>(),
            content: content,
            likes: 0,
        };

        // 3. simpan
        messages.push_back(msg);
        env.storage().instance().set(&MESSAGE_DATA, &messages);

        String::from_str(&env, "Message posted!")
    }

    // ❤️ Like message
    pub fn like_message(env: Env, id: u64) -> String {
        let mut messages: Vec<Message> = env.storage()
            .instance()
            .get(&MESSAGE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..messages.len() {
            let mut msg = messages.get(i).unwrap();

            if msg.id == id {
                msg.likes += 1;
                messages.set(i, msg);

                env.storage().instance().set(&MESSAGE_DATA, &messages);
                return String::from_str(&env, "Message liked!");
            }
        }

        String::from_str(&env, "Message not found")
    }
}