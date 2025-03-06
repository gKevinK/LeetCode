class Solution {
public:
    int minKBitFlips(vector<int>& A, int K) {
        queue<int> q;
        int n = 0;
        for (int i = 0; i < A.size(); ++i) {
            if (q.front() == i)
                q.pop();
            if (!(A[i] ^ (q.size() % 2))) {
                if (i + K > A.size())
                    return -1;
                q.push(i + K);
                ++n;
            }
        }
        return n;
    }
};