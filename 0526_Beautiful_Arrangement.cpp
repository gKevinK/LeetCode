class Solution {
    int a(vector<int> & v, int N, int p) {
        if (p == N) return 1;
        int c = 0;
        for (int i = 1; i <= N; i++) {
            if (i % (p+1) && (p+1) % i) continue;
            int j = 0;
            for (; j < p; j++) {
                if (v[j] == i) break;
            }
            if (j != p) continue;
            v[p] = i;
            c += a(v, N, p+1);
            v[p] = 0;
        }
        return c;
    }
public:
    int countArrangement(int N) {
        vector<int> v(N, 0);
        return a(v, N, 0);
    }
};