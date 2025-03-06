class AllOne {
    list<pair<int, unordered_set<string>>> l;
    unordered_map<string, decltype(l.begin())> m;
public:
    /** Initialize your data structure here. */
    AllOne() {
        
    }
    
    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    void inc(string key) {
        if (m.find(key) == m.end()) {
            if (l.empty() || l.front().first > 1)
                l.insert(l.begin(), { 1, {} });
            l.front().second.insert(key);
            m[key] = l.begin();
        } else {
            auto it2 = m[key];
            it2++;
            if (it2 == l.end() || it2->first > m[key]->first + 1)
                it2 = l.insert(it2, { m[key]->first + 1, {} });
            it2->second.insert(key);
            m[key]->second.erase(key);
            if (m[key]->second.empty())
                l.erase(m[key]);
            m[key] = it2;
        }
    }
    
    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    void dec(string key) {
        if (m.find(key) == m.end()) return;
        if (m[key]->first == 1) {
            m[key]->second.erase(key);
            if (m[key]->second.empty())
                l.erase(m[key]);
            m.erase(key);
        } else {
            auto it = m[key];
            if (it == l.begin())
                l.insert(it, { m[key]->first - 1, {} });
            it--;
            if (it->first < m[key]->first - 1)
                it = l.insert(m[key], { m[key]->first - 1, {} });
            it->second.insert(key);
            m[key]->second.erase(key);
            if (m[key]->second.empty())
                l.erase(m[key]);
            m[key] = it;
        }
    }
    
    /** Returns one of the keys with maximal value. */
    string getMaxKey() {
        return l.empty() ? "" : *l.back().second.begin();
    }
    
    /** Returns one of the keys with Minimal value. */
    string getMinKey() {
        return l.empty() ? "" : *l.front().second.begin();
    }
};

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne obj = new AllOne();
 * obj.inc(key);
 * obj.dec(key);
 * string param_3 = obj.getMaxKey();
 * string param_4 = obj.getMinKey();
 */