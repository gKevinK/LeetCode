class Solution {
public:
    vector<pair<int, int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
        if (nums1.empty() || nums2.empty()) return { };
        using pii = pair<int, int>;
        auto cmp = [&](const pii & l, const pii & r) {
            return nums1[l.first] + nums2[l.second] > nums1[r.first] + nums2[r.second]; };
        priority_queue<pii, vector<pii>, decltype(cmp)> pq(cmp);
        for (int i = 0; i < min((int)nums1.size(), k); i++)
            pq.push({ i, 0 });
        vector<pii> res;
        while (pq.size() && res.size() < k) {
            auto p = pq.top();
            res.push_back({ nums1[p.first], nums2[p.second] });
            pq.pop();
            if (p.second + 1 < nums2.size())
                pq.push({ p.first, p.second + 1 });
        }
        return res;
    }
};