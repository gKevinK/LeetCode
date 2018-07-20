class Solution {
public:
    vector<int> advantageCount(vector<int>& A, vector<int>& B) {
        int lo = 0, hi = A.size() - 1, n = A.size();
        priority_queue<pair<int, int>> q;
        for (int i = 0; i < n; i++)
            q.push({ B[i], i });
        sort(A.begin(), A.end());
        vector<int> R(n, 0);
        while (!q.empty()) {
            int val = q.top().first, idx = q.top().second;
            R[idx] = (val >= A[hi]) ? A[lo++] : A[hi--];
            q.pop();
        }
        return R;
    }
};