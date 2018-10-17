class RandomizedCollection {
    unordered_map<int, unordered_set<int>> m;
    vector<int> v;
    
public:
    /** Initialize your data structure here. */
    RandomizedCollection() {
        
    }
    
    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    bool insert(int val) {
        bool r = (m.find(val) == m.end());
        m[val].insert(v.size());
        v.push_back(val);
        return r;
    }
    
    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    bool remove(int val) {
        if (m.find(val) == m.end()) return false;
        int l = *m[val].begin();
        m[val].erase(l);
        m[v.back()].insert(l);
        m[v.back()].erase(v.size() - 1);
        v[l] = v.back();
        v.pop_back();
        if (m[val].empty())
            m.erase(val);
        return true;
    }
    
    /** Get a random element from the collection. */
    int getRandom() {
        return v[rand() % v.size()];
    }
};

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * RandomizedCollection obj = new RandomizedCollection();
 * bool param_1 = obj.insert(val);
 * bool param_2 = obj.remove(val);
 * int param_3 = obj.getRandom();
 */