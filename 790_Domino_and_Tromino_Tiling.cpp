class Solution {
public:
    int numTilings(int N) {
        vector<int> v(N + 1, 0);
        v[0] = 1;
        for (int i = 1; i <= N; i++) {
            for (int j = 0; j < i; j++) {
                if (i - j == 1 || i - j == 2)
                    v[i] += v[j];
                else
                    v[i] += 2 * v[j] % 1000000007;
                if (v[i] >= 1000000007)
                    v[i] -= 1000000007;
            }
        }
        return v.back();
    }
};