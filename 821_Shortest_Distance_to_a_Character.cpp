class Solution {
public:
    vector<int> shortestToChar(string S, char C) {
        vector<int> vs(S.size(), INT_MAX), vc;
        for (int i = 0; i < S.size(); i++)
            if (S[i] == C) {
                vs[i] = 0;
                vc.push_back(i);
            }
        for (int & i : vc) {
            for (int j = i - 1; j >= 0 && vs[j] > i - j; j--)
                vs[j] = i - j;
            for (int j = i + 1; j < vs.size() && vs[j] > j - i; j++)
                vs[j] = j - i;
        }
        return vs;
    }
};