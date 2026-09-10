// Created by Olgierd Palasz at 2026/09/06 22:12
// leetgo: dev
// https://leetcode.com/problems/design-twitter/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::collections::{BinaryHeap, HashMap, HashSet};
struct Twitter {
    users: HashMap<i32, User>,
    timestamp: i32,
}

struct User {
    following: HashSet<i32>,
    posts: Vec<(i32, i32)>,
}

impl User {
    fn new(id: i32) -> Self {
        Self {
            following: HashSet::from([id]),
            posts: Vec::new(),
        }
    }
}

impl Twitter {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            timestamp: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.users
            .entry(user_id)
            .or_insert_with(|| User::new(user_id))
            .posts
            .push((self.timestamp, tweet_id));
        self.timestamp += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let Some(user) = self.users.get(&user_id) else {
            return Vec::new();
        };

        #[derive(PartialEq, PartialOrd, Ord, Eq)]
        struct HeapEntry {
            timestamp: i32,
            post_id: i32,
            user_id: i32,
            index: usize,
        }

        let mut heap: BinaryHeap<HeapEntry> = user
            .following
            .iter()
            .filter_map(|&user_id| {
                let posts = &self.users.get(&user_id)?.posts;
                let i = posts.len().checked_sub(1)?;
                let (timestamp, post_id) = posts[i];
                Some(HeapEntry {
                    timestamp,
                    post_id,
                    user_id,
                    index: i,
                })
            })
            .collect();

        let mut feed = Vec::with_capacity(10);

        while feed.len() < 10 {
            let Some(heap_entry) = heap.pop() else {
                break;
            };

            feed.push(heap_entry.post_id);

            if heap_entry.index > 0 {
                let (timestamp, post_id) =
                    self.users.get(&heap_entry.user_id).unwrap().posts[heap_entry.index - 1];
                heap.push(HeapEntry {
                    timestamp,
                    post_id,
                    user_id: heap_entry.user_id,
                    index: heap_entry.index - 1,
                });
            }
        }

        feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert_with(|| User::new(follower_id))
            .following
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(user) = self.users.get_mut(&follower_id) {
            user.following.remove(&followee_id);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = Twitter::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "postTweet" => {
                let method_params = split_array(&params[i])?;
                let user_id: i32 = deserialize(&method_params[0])?;
                let tweet_id: i32 = deserialize(&method_params[1])?;
                obj.post_tweet(user_id, tweet_id);
                output.push("null".to_string());
            }
            "getNewsFeed" => {
                let method_params = split_array(&params[i])?;
                let user_id: i32 = deserialize(&method_params[0])?;
                let ans: Vec<i32> = obj.get_news_feed(user_id).into();
                output.push(serialize(ans)?);
            }
            "follow" => {
                let method_params = split_array(&params[i])?;
                let follower_id: i32 = deserialize(&method_params[0])?;
                let followee_id: i32 = deserialize(&method_params[1])?;
                obj.follow(follower_id, followee_id);
                output.push("null".to_string());
            }
            "unfollow" => {
                let method_params = split_array(&params[i])?;
                let follower_id: i32 = deserialize(&method_params[0])?;
                let followee_id: i32 = deserialize(&method_params[1])?;
                obj.unfollow(follower_id, followee_id);
                output.push("null".to_string());
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
