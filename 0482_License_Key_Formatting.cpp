class Solution {
public:
    string licenseKeyFormatting(string S, int K) {
        int count = 0;
        for (char & c : S)
            if (isalpha(c) || isdigit(c))
                count++;
        string r;
        for (int i = 0; count; i++, count--) {
            while (!isalpha(S[i]) && !isdigit(S[i])) i++;
            if (!r.empty() && count % K == 0)
                r += '-';
            r += isalpha(S[i]) ? toupper(S[i]) : S[i];
        }
        return r;
    }
};