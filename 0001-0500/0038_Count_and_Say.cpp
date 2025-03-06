class Solution {
public:
    string countAndSay(int n) {
        static vector<string> cache;
        if (cache.empty()) cache.push_back("1");
        while (cache.size() < n) {
            string & s1 = cache.back();
            string s2;
            for (int i = 0; i < s1.size(); ) {
                int j = i;
                while (j < s1.size() && s1[j] == s1[i]) j++;
                s2 += (j - i) + '0';
                s2 += s1[i];
                i = j;
            }
            cache.push_back(s2);
        }
        return cache[n - 1];
    }
};