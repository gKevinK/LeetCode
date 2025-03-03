class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> map;
        for (int & i : nums)
            map[i]++;
        priority_queue<pair<int, int>> q;
        for (auto & p : map) 
            q.push({ p.second, p.first });
        vector<int> r;
        while (r.size() < k) {
            r.push_back(q.top().second);
            q.pop();
        }
        return r;
    }
};