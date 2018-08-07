class Solution {
public:
    int reachableNodes(vector<vector<int>>& edges, int M, int N) {
        unordered_map<int, unordered_map<int, int>> es;
        for (auto & e : edges)
            es[e[0]][e[1]] = es[e[1]][e[0]] = e[2];
        vector<int> ms(N, -1);
        priority_queue<pair<int, int>> pq;
        pq.push({ M, 0 });
        while (!pq.empty()) {
            int m = pq.top().first, n = pq.top().second;
            pq.pop();
            if (ms[n] >= 0) continue;
            ms[n] = m;
            for (auto & p : es[n]) {
                if (m - p.second - 1 >= 0 && ms[p.first] == -1)
                    pq.push({ m - p.second - 1, p.first });
            }
        }
        int count = 0;
        for (int i : ms)
            count += (i >= 0);
        for (auto & e : edges)
            count += min(e[2], max(ms[e[0]], 0) + max(ms[e[1]], 0));
        return count;
    }
};