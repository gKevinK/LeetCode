class Solution {
public:
    int maxSubarraySumCircular(vector<int>& A) {
        int n = A.size(), sum = 0, s1 = 0, s2 = 0, m1 = INT_MIN, m2 = INT_MAX;
        for (int i : A) {
            sum += i;
            s1 = max(s1 + i, i);
            s2 = min(s2 + i, i);
            m1 = max(m1, s1);
            m2 = min(m2, s2);
        }
        return m1 > 0 ? max(m1, sum - m2) : m1;
    }
};