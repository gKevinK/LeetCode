class Solution {
public:
    double mincostToHireWorkers(vector<int>& quality, vector<int>& wage, int K) {
        vector<pair<double, int>> workers;
        for (int i = 0; i < quality.size(); ++i) {
            workers.push_back({ static_cast<double>(wage[i]) / quality[i], quality[i] });
        }
        sort(workers.begin(), workers.end());
        priority_queue<int> pq;
        int q = 0;
        double r = 1e12;
        for (auto & w : workers) {
            pq.push(w.second);
            q += w.second;
            if (pq.size() > K) {
                q -= pq.top();
                pq.pop();
            }
            if (pq.size() == K) {
                r = min(r, q * w.first);
            }
        }
        return r;
    }
};