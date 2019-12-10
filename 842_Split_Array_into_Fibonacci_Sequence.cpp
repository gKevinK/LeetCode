class Solution {
public:
    vector<int> splitIntoFibonacci(string S) {
        for (int i = 1; i < S.size(); ++i) {
            for (int j = i + 1; j < S.size(); ++j) {
                if (i > 1 && S[0] == '0') continue;
                if (j - i > 1 && S[i] == '0') continue;
                vector<int> v;
                long t = 0;
                int k = 0;
                for (; k < i && t < INT_MAX; ++k) t = t * 10 + (S[k] - '0');
                if (t > INT_MAX) continue;
                v.push_back(t);
                t = 0;
                for (; k < j && t < INT_MAX; ++k) t = t * 10 + (S[k] - '0');
                if (t > INT_MAX) continue;
                v.push_back(t);
                while (k < S.size()) {
                    t = S[k++] - '0';
                    long target = static_cast<long>(v[v.size() - 2]) + v[v.size() - 1];
                    if (target > INT_MAX) break;
                    if (t)
                        for (; t < target && k < S.size(); ++k) t = t * 10 + (S[k] - '0');
                    if (t == target) {
                        v.push_back(t);
                        if (k == S.size()) return v;
                    } else break;
                }
            }
        }
        return {};
    }
};