// class Solution {
//     int n;
//     vector<int> dp;
    
//     int f(vector<string> & sts, string & tg, int t) {
//         if (t == 0) return 0;
//         if (dp[t] != 0) return dp[t];
//         int r = 1000;
//         for (string & st : sts) {
//             int t2 = t;
//             for (char & c : st) {
//                 for (int i = 0, m = n >> 1; i < tg.size(); ++i, m >>= 1)
//                     if (c == tg[i] && (t2 & m)) {
//                         t2 ^= m;
//                         break;
//                     }
//             }
//             if (t2 != t) {
//                 int tr = f(sts, tg, t2);
//                 if (tr != -1)
//                     r = min(r, 1 + tr);
//             }
//         }
//         dp[t] = r == 1000 ? -1 : r;
//         return dp[t];
//     }
// public:
//     int minStickers(vector<string>& stickers, string target) {
//         n = 1 << target.size();
//         dp = vector<int>(n, 0);
//         return f(stickers, target, n - 1);
//     }
// };

class Solution {    
    int f(unordered_map<string, int> & dp, vector<vector<int>> & mp, string t) {
        if (dp.find(t) != dp.end()) return dp[t];
        int r = 1000;
        vector<int> tp(26, 0);
        for (char & c : t)
            tp[c - 'a']++;
        for (auto & m : mp) {
            if (m[t[0] - 'a'] == 0)
                continue;
            string s;
            for (int i = 0; i < 26; ++i)
                if (m[i] < tp[i])
                    s += string(tp[i] - m[i], 'a' + i);
            int tr = f(dp, mp, s);
            if (tr != -1)
                r = min(r, 1 + tr);
        }
        dp[t] = r == 1000 ? -1 : r;
        return dp[t];
    }
public:
    int minStickers(vector<string>& stickers, string target) {
        unordered_map<string, int> dp;
        dp[""] = 0;
        vector<vector<int>> mp(stickers.size(), vector<int>(26, 0));
        for (int i = 0; i < stickers.size(); ++i)
            for (char & c : stickers[i])
                ++mp[i][c - 'a'];
        return f(dp, mp, target);
    }
};