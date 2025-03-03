class Solution {
    vector<int> vc;
    vector<int> num;
    
    void f(int i, char ic, string s) {
        num[i] = vc[ic - 'a'];
        for (char & c : s)
            vc[c - 'a'] -= num[i];
    }
public:
    string originalDigits(string s) {
        vc = vector<int>(26, 0);
        num = vector<int>(10, 0);
        for (char & c : s)
            vc[c - 'a']++;
        f(0, 'z', "zero");
        f(2, 'w', "two");
        f(4, 'u', "four");
        f(6, 'x', "six");
        f(8, 'g', "eight");
        f(1, 'o', "one");
        f(3, 'r', "three");
        f(7, 's', "seven");
        f(5, 'v', "five");
        f(9, 'i', "nine");
        string r;
        for (int i = 0; i <= 9; i++)
            for (int j = 0; j < num[i]; j++)
                r += i + '0';
        return r;
    }
};