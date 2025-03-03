class TimeMap {
    unordered_map<string, map<int, string>> m;
    
public:
    /** Initialize your data structure here. */
    TimeMap() {
        
    }
    
    void set(string key, string value, int timestamp) {
        m[key][timestamp] = value;
    }
    
    string get(string key, int timestamp) {
        if (m.find(key) == m.end()) return "";
        auto & m2 = m[key];
        auto p = m2.upper_bound(timestamp);
        if (p == m2.begin()) return "";
        return prev(p)->second;
    }
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */