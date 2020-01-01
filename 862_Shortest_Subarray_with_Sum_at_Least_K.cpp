class Solution {
public:
    int shortestSubarray(vector<int>& A, int K) {
        int n = A.size(), r = A.size() + 1;
        vector<int> s = { 0 };
        deque<int> dq;
        for (int i = 0; i < n + 1; ++i) {
            if (i > 0)
                s.push_back(s.back() + A[i - 1]);
            if (s[i] >= K)
                r = min(r, i);
            while (!dq.empty() && s[i] - s[dq.front()] >= K) {
                r = min(r, i - dq.front());
                dq.pop_front();
            }
            while (!dq.empty() && s[dq.back()] > s[i])
                dq.pop_back();
            dq.push_back(i);
        }
        return r < A.size() + 1 ? r : -1;
    }
};