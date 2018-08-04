class LRUCache {
    unordered_map<int, pair<int, list<int>::iterator>> m;
    list<int> l;
    int capa;
    
    void touch(int key) {
        l.erase(m[key].second);
        l.push_front(key);
        m[key].second = l.begin();
    }
    
public:
    LRUCache(int capacity) {
        capa = capacity;
    }
    
    int get(int key) {
        if (m.find(key) == m.end()) return -1;
        int res = m[key].first;
        touch(key);
        return res;
    }
    
    void put(int key, int value) {
        if (m.find(key) == m.end()) {
            l.push_front(key);
            m[key] = { value, l.begin() };
            if (l.size() > capa) {
                m.erase(l.back());
                l.pop_back();
            }
        } else {
            m[key].first = value;
            touch(key);
        }
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */