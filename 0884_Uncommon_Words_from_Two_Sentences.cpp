class Solution {
public:
    vector<string> uncommonFromSentences(string A, string B) {
        unordered_map<string, int> ma, mb;
        for (int i = 0, j = 0; i < A.size(); j = i) {
            while (j < A.size() && A[j] != ' ') j++;
            ma[A.substr(i, j - i)]++;
            i = j;
            while (i < A.size() && A[i] == ' ') i++;
        }
        for (int i = 0, j = 0; i < B.size(); j = i) {
            while (j < B.size() && B[j] != ' ') j++;
            mb[B.substr(i, j - i)]++;
            i = j;
            while (i < B.size() && B[i] == ' ') i++;
        }
        vector<string> r;
        for (auto  & p : ma)
            if (p.second == 1 && mb.find(p.first) == mb.end())
                r.push_back(p.first);
        for (auto  & p : mb)
            if (p.second == 1 && ma.find(p.first) == ma.end())
                r.push_back(p.first);
        return r;
    }
};