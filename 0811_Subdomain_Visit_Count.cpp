class Solution {
public:
    vector<string> subdomainVisits(vector<string>& cpdomains) {
        unordered_map<string, int> m;
        for (string & s : cpdomains) {
            int n = 0, i;
            for (i = 0; i < s.size(); i++) {
                if (n <= 0 && s[i] >= '0' && s[i] <= '9')
                    n = n * 10 - (s[i] - '0');
                else if (s[i] == ' ')
                    n = -n;
                else {
                    m[s.substr(i)] += n;
                    while (i < s.size() && s[i] != '.')
                        i++;
                }
            }
        }
        vector<string> r;
        r.reserve(m.size());
        for (auto & p : m) {
            r.emplace_back(to_string(p.second) + " " + p.first);
        }
        return r;
    }
};