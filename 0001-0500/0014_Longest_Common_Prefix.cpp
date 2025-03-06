class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        string p = "";
        if (strs.size() == 0) return p;
        for (int i = 0; true; p += strs[0][i], ++i) {
            for (auto str : strs) {
                if (str.size() == i || str[i] != strs[0][i])
                    return p;
            }
        }
        return p;
    }
};