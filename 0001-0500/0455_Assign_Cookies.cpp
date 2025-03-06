class Solution {
public:
    int findContentChildren(vector<int>& g, vector<int>& s) {
        sort(g.begin(), g.end());
        sort(s.begin(), s.end());
        int res = 0, st = 0;
        for (int & c : g) {
            while (st < s.size() && s[st] < c) st++;
            if (st == s.size()) break;
            else {
                st++;
                res++;
            }
        }
        return res;
    }
};