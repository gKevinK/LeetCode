class Solution {
public:
    int maxProduct(vector<string>& words) {
        unordered_map<int, int> m;
        int r = 0;
        for (string & w : words) {
            int mask = 0;
            for (char & c : w)
                mask |= 1 << (c - 'a');
            m[mask] = max(m[mask], (int)w.size());
            for (auto & p : m)
                if (!(mask & p.first))
                    r = max(r, (int)w.size() * p.second);
        }
        return r;
    }
};