class Solution {
public:
    vector<string> topKFrequent(vector<string>& words, int k) {
        unordered_map<string, int> m;
        for (auto & w : words)
            m[w]++;
        using pis = pair<int, string>;
        auto cmp = [](const pis & a, const pis & b) {
            return a.first != b.first ? a.first > b.first : a.second < b.second;
        };
        priority_queue<pis, vector<pis>, decltype(cmp)> pq(cmp);
        for (auto & p : m) {
            if (pq.size() < k || pq.top().first <= p.second)
                pq.push({ p.second, p.first });
            if (pq.size() > k)
                pq.pop();
        }
        vector<string> r;
        while (!pq.empty()) {
            r.push_back(pq.top().second);
            pq.pop();
        }
        return vector<string>(r.rbegin(), r.rend());
    }
};