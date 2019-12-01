class Solution {
public:
    string maskPII(string S) {
        string r;
        if (isalpha(S[0])) {
            r += tolower(S[0]);
            r += "*****";
            int i = 2;
            while (S[i] != '@') ++i;
            r += tolower(S[i - 1]);
            for (; i < S.size(); ++i)
                r += isalpha(S[i]) ? tolower(S[i]) : S[i];
        } else {
            int n = 0;
            for (char & c : S) {
                if (isdigit(c)) ++n;
            }
            if (n > 10) {
                r += '+';
                for (int i = n - 10; i; --i) r += '*';
                r += '-';
            }
            r += "***-***-";
            for (char & c : S) {
                if (isdigit(c)) {
                    --n;
                    if (n < 4) r += c;
                }
            }
        }
        return r;
    }
};