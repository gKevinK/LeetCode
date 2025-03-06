class Solution {
public:
    vector<string> wordSubsets(vector<string>& A, vector<string>& B) {
        vector<int> vc1(26, 0), vc2(26);
        for (string & b : B) {
            fill(vc2.begin(), vc2.end(), 0);
            for (char & c : b)
                vc2[c - 'a']++;
            for (int i = 0; i < 26; i++)
                vc1[i] = max(vc1[i], vc2[i]);
        }
        vector<string> res;
        for (string & a : A) {
            fill(vc2.begin(), vc2.end(), 0);
            for (char & c : a)
                vc2[c - 'a']++;
            bool u = true;
            for (int i = 0; i < 26; i++)
                if (vc1[i] > vc2[i]) {
                    u = false;
                    break;
                }
            if (u) res.push_back(a);
        }
        return res;
    }
};