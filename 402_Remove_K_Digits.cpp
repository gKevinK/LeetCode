class Solution {
public:
    string removeKdigits(string num, int k) {
        if (k == num.size()) return "0";
        string r;
        int n = num.size();
        for (int i = 0; i < n; i++) {
            while (!r.empty() && r.back() > num[i] && n - i + r.size() > n - k)
                r.pop_back();
            if (r.size() < n - k)
                r.push_back(num[i]);
        }
        int i = 0;
        while (i < r.size() && r[i] == '0') i++;
        if (i == r.size()) return "0";
        else return r.substr(i);
    }
};