class Solution {
public:
    bool isHappy(int n) {
        unordered_set<int> s;
        int n1 = 0, n2 = n;
        while (n2 != 1) {
            n1 = n2;
            n2 = 0;
            while (n1) {
                n2 += (n1 % 10) * (n1 % 10);
                n1 /= 10;
            }
            if (s.find(n2) != s.end())
                return false;
            s.insert(n2);
        }
        return true;
    }
};