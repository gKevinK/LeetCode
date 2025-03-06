class Solution {
public:
    string orderlyQueue(string S, int K) {
        if (K == 1) {
            string r = S;
            for (int i = 0; i < S.size(); i++) {
                if (S[i] <= r[0]) {
                    string r2 = S.substr(i) + S.substr(0, i);
                    if (r2 < r)
                        r = r2;
                }
            }
            return r;
        } else {
            sort(S.begin(), S.end());
            return S;
        }
    }
};