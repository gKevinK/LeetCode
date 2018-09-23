class Solution {
public:
    string frequencySort(string s) {
        vector<pair<int, char>> n(128, { 0, 0 });
        for (char c = 0; c >= 0; c++)
            n[c].second = c;
        for (char & c : s)
            n[c].first++;
        sort(n.rbegin(), n.rend());
        string r;
        for (char c = 0; c >= 0 && n[c].first > 0; c++)
            for (int i = 0; i < n[c].first; i++)
                r += n[c].second;
        return r;
    }
};