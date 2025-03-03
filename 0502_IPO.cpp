class Solution {
public:
    int findMaximizedCapital(int k, int W, vector<int>& Profits, vector<int>& Capital) {
        vector<pair<int, int>> v;
        for (int i = 0; i < Profits.size(); i++)
            v.push_back({ Capital[i], Profits[i] });
        sort(v.begin(), v.end());
        int cur = 0, curW = W;
        priority_queue<int> pq;
        for (int i = 0; i < k; i++) {
            while (cur < v.size() && v[cur].first <= curW) {
                pq.push(v[cur].second);
                cur++;
            }
            if (pq.empty()) break;
            curW += pq.top();
            pq.pop();
        }
        return curW;
    }
};