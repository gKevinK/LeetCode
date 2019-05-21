class RangeModule {
    map<int, int> m;

public:
    RangeModule() {
        
    }
    
    void addRange(int left, int right) {
        m[left] = max(right, m[left]);
        auto p = m.find(left);
        if (p != m.begin()) {
            p--;
            if (p->second < left)
                p++;
        }
        auto u = m.upper_bound(p->first);
        while (u != m.end() && u->first <= p->second) {
            p->second = max(p->second, u->second);
            m.erase(u);
            u = m.upper_bound(p->first);
        }
    }
    
    bool queryRange(int left, int right) {
        auto p = m.upper_bound(left);
        if (p == m.begin()) return false;
        p--;
        return p->second >= right;
    }
    
    void removeRange(int left, int right) {
        auto p = m.lower_bound(left);
        if (p != m.begin()) {
            p--;
            if (p->second > right)
                m[right] = p->second;
            p->second = min(p->second, left);
            p++;
        }
        while (p != m.end() && p->first < right) {
            int k = p->first;
            if (right < p->second) {
                m[right] = p->second;
            }
            p++;
            m.erase(k);
        }
    }
};

/**
 * Your RangeModule object will be instantiated and called as such:
 * RangeModule* obj = new RangeModule();
 * obj->addRange(left,right);
 * bool param_2 = obj->queryRange(left,right);
 * obj->removeRange(left,right);
 */