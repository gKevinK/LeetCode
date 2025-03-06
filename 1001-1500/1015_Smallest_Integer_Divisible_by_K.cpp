class Solution {
public:
    int smallestRepunitDivByK(int K) {
        vector<bool> visited(K, false);
        int test = 0, len = 0;
        while (true) {
            test = (test * 10 + 1) % K;
            len++;
            if (test == 0)
                return len; 
            if (visited[test])
                return -1;
            visited[test] = true;
        }
    }
};