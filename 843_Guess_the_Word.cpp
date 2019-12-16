/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * class Master {
 *   public:
 *     int guess(string word);
 * };
 */
class Solution {
    int match(const string & a, const string & b) {
        int m = 0;
        for (int i = 0; i < 6; ++i) {
            if (a[i] == b[i]) ++m;
        }
        return m;
    }
public:
    void findSecretWord(vector<string>& wordlist, Master& master) {
        vector<string> v = wordlist;
        for (int i = 0; i < 10; ++i) {
            unordered_map<string, int> m;
            string s;
            int c = v.size() + 1;
            for (string & w1 : v) {
                m[w1] = 0;
                for (string & w2 : v) {
                    if (match(w1, w2) == 0)
                        m[w1]++;
                }
            }
            for (auto & p : m) {
                if (p.second < c) {
                    c = p.second;
                    s = p.first;
                }
            }
            int r = master.guess(s);
            if (r == 6) return;
            vector<string> v2;
            for (string & w : v) {
                if (match(s, w) == r)
                    v2.push_back(w);
            }
            v.swap(v2);
        }
    }
};