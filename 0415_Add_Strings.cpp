class Solution {
public:
    string addStrings(string num1, string num2) {
        string s;
        int carry = 0;
        for (int i1 = num1.size() - 1, i2 = num2.size() - 1; i1 >= 0 || i2 >= 0 || carry > 0; i1--, i2--) {
            int c1 = i1 < 0 ? 0 : num1[i1] - '0';
            int c2 = i2 < 0 ? 0 : num2[i2] - '0';
            int sum = carry + c1 + c2;
            s.push_back(sum % 10 + '0');
            carry = sum / 10;
        }
        for (int i = 0, j = s.size() - 1; i < j; i++, j--)
            swap(s[i], s[j]);
        return s;
    }
};