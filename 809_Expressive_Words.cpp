class Solution {
public:
    int expressiveWords(string S, vector<string>& words) {
        int r = 0;
        for (auto & w : words) {
            int i = 0, j = 0;
            while (i < S.size() && j < w.size()) {
                int ci = i, cj = j;
                if (S[i] != w[j]) break;
                while (ci < S.size() && S[ci] == S[i]) ci++;
                while (cj < w.size() && w[cj] == w[j]) cj++;
                if ((ci - i > cj - j && ci - i < 3) || ci - i < cj - j) break;
                i = ci;
                j = cj;
            }
            if (i == S.size() && j == w.size())
                r++;
        }
        return r;
    }
};