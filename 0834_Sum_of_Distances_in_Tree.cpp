class Solution {
    vector<int> num, sod;
    vector<vector<int>> n2e, es;
    
    pair<int, int> calc(int ei) {
        if (num[ei]) return { num[ei], sod[ei] };
        int from = es[ei][0], node = es[ei][1];
        int n = 1, s = 1;
        for (int i : n2e[node]) {
            if  (from == es[i][1]) continue;
            auto p = calc(i);
            n += p.first;
            s += p.second + p.first;
        }
        num[ei] = n;
        sod[ei] = s;
        return { n, s };
    }
public:
    vector<int> sumOfDistancesInTree(int N, vector<vector<int>>& edges) {
        n2e = vector<vector<int>>(N);
        es = vector<vector<int>>();
        for (auto & e : edges) {
            n2e[e[0]].push_back(es.size());
            es.push_back({ e[0], e[1] });
            n2e[e[1]].push_back(es.size());
            es.push_back({ e[1], e[0] });
        }
        num = sod = vector<int>(N * 2, 0);
        for (int i = 0; i < es.size(); ++i)
            calc(i);
        vector<int> res;
        for (int i = 0; i < N; ++i) {
            int r = 0;
            for (int j : n2e[i])
                r += sod[j];
            res.push_back(r);
        }
        return res;
    }
};