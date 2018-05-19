class MapSum {
    unordered_map<string, int> m, m2;
public:
    /** Initialize your data structure here. */
    MapSum() {
        
    }
    
    void insert(string key, int val) {
        val -= m2[key];
        m2[key] += val;
        for (int i = 1; i <= key.size(); i++) {
            m[key.substr(0, i)] += val;
        }
    }
    
    int sum(string prefix) {
        return m[prefix];
    }
};

/**
 * Your MapSum object will be instantiated and called as such:
 * MapSum obj = new MapSum();
 * obj.insert(key,val);
 * int param_2 = obj.sum(prefix);
 */