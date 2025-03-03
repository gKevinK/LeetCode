class Solution {
public:
    int numJewelsInStones(string J, string S) {
        vector<bool> v(256, false);
        int n = 0;
        for (const char & c : J)
            v[c] = true;
        for (const char & c : S)
            if (v[c])
                n++;
        return n;
    }
};