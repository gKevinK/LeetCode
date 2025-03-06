class Solution {
public:
    string multiply(string num1, string num2) {
        reverse(num1.begin(), num1.end());
        reverse(num2.begin(), num2.end());
        string res;
        for (int i = 0; i < num1.size(); i++) {
            for (int j = 0; j < num2.size(); j++) {
                int t = (num1[i] - '0') * (num2[j] - '0');
                for (int k = i + j; t; k++) {
                    while (res.size() <= k) res.push_back('0');
                    t += res[k] - '0';
                    res[k] = t % 10 + '0';
                    t /= 10;
                }
            }
        }
        reverse(res.begin(), res.end());
        if (res.empty()) res = "0";
        return res;
    }
};