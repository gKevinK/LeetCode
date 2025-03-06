class Solution {
public:
    string decodeAtIndex(string S, int K) {
        long long i = 0, l = 0, n = S.size();
        for (; i < n && l < K; i++) {
            if (isdigit(S[i])) l *= S[i] - '0';
            else l++;
        }
        for (i--; i >= 0; i--) {
            if (isdigit(S[i])) {
                l /= S[i] - '0';
                K = (K - 1) % l + 1;
            } else {
                if (l == K) return string(1, S[i]);
                l--;
            }
        }
    }
};