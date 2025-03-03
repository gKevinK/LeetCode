class Solution {
public:
    bool isNStraightHand(vector<int>& hand, int W) {
        map<int, int> m;
        for (int & i : hand)
            m[i]++;
        for (auto & p : m) {
            int n = m[p.first];
            if (n == 0)
                continue;
            for (int i = p.first; i < W + p.first; i++) {
                m[i] -= n;
                if (m[i] < 0)
                    return false;
            }
        }
        return true;
    }
};