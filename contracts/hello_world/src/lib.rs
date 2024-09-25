#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, String, symbol_short, Symbol};

// Structure for tracking a post on the decentralized social media platform.
#[contracttype]
#[derive(Clone)]
pub struct Post {
    pub post_id: u64,          // unique ID for the post
    pub title: String,         // title of the post
    pub description: String,   // content/description of the post
    pub creator: String,       // creator's name or address
    pub created_at: u64,       // timestamp of post creation
    pub is_active: bool,       // indicates if the post is active (not deleted)
}

// Mapping posts by their post IDs.
#[contracttype]
pub enum Postbook {
    Post(u64),
}

// Unique identifier for the next post.
const COUNT_POST: Symbol = symbol_short!("C_POST");

// Main contract for the decentralized social media platform.
#[contract]
pub struct SocialMediaPlatformContract;

#[contractimpl]
impl SocialMediaPlatformContract {
    // Function to create a new post.
    pub fn create_post(env: Env, title: String, description: String, creator: String) -> u64 {
        // Fetching the current count of posts, or default to 0 if none exist yet.
        let mut count_post: u64 = env.storage().instance().get(&COUNT_POST).unwrap_or(0);
        count_post += 1;

        // Getting the current timestamp for post creation.
        let timestamp = env.ledger().timestamp();

        // Creating a new post instance.
        let new_post = Post {
            post_id: count_post,
            title,
            description,
            creator,
            created_at: timestamp,
            is_active: true,
        };

        // Storing the new post.
        env.storage().instance().set(&Postbook::Post(count_post), &new_post);

        // Updating the post count.
        env.storage().instance().set(&COUNT_POST, &count_post);

        // Log the post creation event.
        log!(&env, "Post Created with ID: {}", count_post);

        // Return the post ID.
        count_post
    }

    // Function to view an existing post by post ID.
    pub fn view_post(env: Env, post_id: u64) -> Post {
        let key = Postbook::Post(post_id);
        env.storage().instance().get(&key).unwrap_or(Post {
            post_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            creator: String::from_str(&env, "Unknown"),
            created_at: 0,
            is_active: false,
        })
    }

    // Function to deactivate (delete) a post by marking it inactive.
    pub fn delete_post(env: Env, post_id: u64) {
        let mut post = Self::view_post(env.clone(), post_id);
        if post.is_active {
            post.is_active = false;
            env.storage().instance().set(&Postbook::Post(post_id), &post);
            log!(&env, "Post with ID {} is now deactivated.", post_id);
        } else {
            log!(&env, "Post with ID {} does not exist or is already inactive.", post_id);
        }
    }
}
