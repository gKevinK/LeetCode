class Solution {
public:
    string toGoatLatin(string S) {
        string r, v = "aeiouAEIOU";
        int c = 0;
        for (int i = 0; i < S.size(); i++) {
            if (i != 0)
                r.push_back(' ');
            int i0 = i;
            while (i < S.size() && S[i] != ' ') i++;
            if (v.find(S[i0]) != string::npos)
                r.push_back(S[i0]);
            for (int j = i0 + 1; j < i; j++)
                r.push_back(S[j]);
            if (v.find(S[i0]) == string::npos)
                r.push_back(S[i0]);
            r.push_back('m');
            r.push_back('a');
            for (int j = 0; j <= c; j++)
                r.push_back('a');
            c++;
        }
        
        return r;
    }
};