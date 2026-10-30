use std::collections::HashMap;
use std::collections::BTreeSet;

struct AuctionSystem {
    bid: HashMap<(i32, i32), i32>,
    item: HashMap<i32, BTreeSet<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuctionSystem {

    fn new() -> Self {
        Self {
            bid: HashMap::new(),
            item: HashMap::new()
        }
    }
    
    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        if let Some(&b) = self.bid.get(&(user_id, item_id)) {
            self.item.get_mut(&item_id).unwrap().remove(&(b, user_id));
        }
        self.bid.insert((user_id, item_id), bid_amount);
        self.item.entry(item_id).or_insert(BTreeSet::new()).insert((bid_amount, user_id));
    }
    
    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }
    
    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        if let Some(b) = self.bid.get(&(user_id, item_id)) {
            self.item.get_mut(&item_id).unwrap().remove(&(*b, user_id));
        }
        self.bid.remove(&(user_id, item_id));
    }
    
    fn get_highest_bidder(&self, item_id: i32) -> i32 {
        if let Some(t) = self.item.get(&item_id) {
            t.last().unwrap_or(&(-1, -1)).1
        } else {
            -1
        }
    }
}

/**
 * Your AuctionSystem object will be instantiated and called as such:
 * let obj = AuctionSystem::new();
 * obj.add_bid(userId, itemId, bidAmount);
 * obj.update_bid(userId, itemId, newAmount);
 * obj.remove_bid(userId, itemId);
 * let ret_4: i32 = obj.get_highest_bidder(itemId);
 */