class Solution {
public:
    int largestSumAfterKNegations(vector<int>& A, int K) {
        priority_queue<int> pq;
        int sum = 0, m = 1000;
        for (int & i : A) {
            sum += i;
            m = min(m, abs(i));
            if (i < 0)
                pq.push(i);
            if (pq.size() > K)
                pq.pop();
        }
        if (K > pq.size() && ((K - pq.size()) % 2))
            sum -= 2 * m;
        while (!pq.empty()) {
            sum -= 2 * pq.top();
            pq.pop();
        }
        return sum;
    }
};