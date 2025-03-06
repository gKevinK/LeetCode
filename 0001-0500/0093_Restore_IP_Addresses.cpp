class Solution {
public:
    vector<string> restoreIpAddresses(string s) {
        vector<string> r;
        int n = s.size();
        for (int a = 1; a <= 3 && a <= n - 3; a++) {
            int d1 = stoi(s.substr(0, a));
            if (d1 >= 256) break;
            for (int b = a + 1; b <= a + 3 && b <= n - 2; b++) {
                int d2 = stoi(s.substr(a, b - a));
                if (d2 >= 256) break;
                for (int c = b + 1; c <= b + 3 && c <= n - 1; c++) {
                    int d3 = stoi(s.substr(b, c - b));
                    int d4 = stoi(s.substr(c, min(3, n - c)));
                    if (d3 >= 256 || d4 >= 256) continue;
                    string ip = to_string(d1) + "." + to_string(d2) + "." + to_string(d3) + "." + to_string(d4);
                    if (ip.size() == s.size() + 3)
                        r.push_back(ip);
                }
            }
        }
        return r;
    }
};