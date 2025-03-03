class Solution {
    unordered_map<int, bool> m;
    
    bool can(vector<int>& stones, int pos, int k) {
        int key = (pos << 20) | k;
        if (m.find(key) != m.end()) return m[key];
        for (int i = pos + 1; i < stones.size(); i++) {
            int gap = stones[i] - stones[pos];
            if (gap < k - 1) continue;
            if (gap > k + 1) return m[key] = false;
            if (can(stones, i, gap)) return m[key] = true;
        }
        return m[key] = pos + 1 == stones.size();
    }
    
public:
    bool canCross(vector<int>& stones) {
        m = {};
        return can(stones, 0, 0);
    }
};