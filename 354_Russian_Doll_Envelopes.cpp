class Solution {
public:
    int maxEnvelopes(vector<pair<int, int>>& envelopes) {
        sort(envelopes.begin(), envelopes.end(), [](auto & p1, auto & p2) {
            return p1.first < p2.first || (p1.first == p2.first && p1.second > p2.second); });
        vector<int> v;
        for (auto & p : envelopes) {
            auto it = lower_bound(v.begin(), v.end(), p.second);
            if (it == v.end()) v.push_back(p.second);
            else *it = p.second;
        }
        return v.size();
    }
};