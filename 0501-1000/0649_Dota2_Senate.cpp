class Solution {
public:
    string predictPartyVictory(string senate) {
        int n = senate.size(), sr = 0, sd = 0;
        vector<bool> banned(n, false);
        for (int i = 0; true; ++i) {
            if (banned[i % n]) continue;
            bool isR = senate[i % n] == 'R';
            sd = max(sd, i);
            sr = max(sr, i);
            int j = isR ? sd : sr;
            for (++j; j < i + n ; ++j) {
                if (banned[j % n] || senate[j % n] == senate[i % n]) continue;
                banned[j % n] = true;
                (isR ? sd : sr) = j;
                break;
            }
            if (j == i + n) return isR ? "Radiant" : "Dire";
        }
        return "";
    }
};