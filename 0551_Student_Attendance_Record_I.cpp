class Solution {
public:
    bool checkRecord(string s) {
        int ca = 0, cl = 0;
        for (char c : s) {
            if (c == 'A')
                if (++ca > 1)
                    return false;
            if (c == 'L') {
                if (++cl > 2)
                    return false;
            } else
                cl = 0;
        }
        return true;
    }
};