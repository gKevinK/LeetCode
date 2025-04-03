class Solution {
public:
    int kthGrammar(int N, int K) {
        stack<int> s;
        for (int i = N; i > 1; i--) {
            s.push((K + 1) % 2);
            K = (K + 1) / 2;
        }
        int r = 0;
        while (!s.empty()) {
            r = s.top() ^ r;
            s.pop();
        }
        return r;
    }
};