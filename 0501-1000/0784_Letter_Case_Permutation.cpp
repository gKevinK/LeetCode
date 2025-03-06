class Solution {
public:
    vector<string> letterCasePermutation(string S) {
        vector<string> v;
        vector<int> vi;
        for (int i = 0; i < S.size(); i++) {
            if (S[i] >= 'A' && S[i] <= 'Z')
                S[i] = S[i] - 'A' + 'a';
            if (S[i] >= 'a' && S[i] <= 'z')
                vi.push_back(i);
        }
        vector<int> vs(vi.size(), 0);
        v.push_back(S);
        while (true) {
            int carry = 1;
            for (int i = vs.size() - 1; i >= 0; i--) {
                carry += vs[i];
                vs[i] = carry % 2;
                carry /= 2;
            }
            if (carry) break;
            v.push_back(S);
            for (int i = 0; i < vi.size(); i++) {
                if (vs[i])
                    v.back()[vi[i]] += 'A' - 'a';
            }
        }
        return v;
    }
};