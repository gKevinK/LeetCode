class Solution {
public:
    char findTheDifference(string s, string t) {
        char c = 0;
        for (char & i : s)
            c ^= i;
        for (char & i : t)
            c ^= i;
        return c;
    }
};