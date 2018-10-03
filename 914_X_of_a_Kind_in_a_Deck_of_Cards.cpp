class Solution {
    int gcd(int a, int b) {
        while (b) {
            a %= b;
            swap(a, b);
        }
        return a;
    }
public:
    bool hasGroupsSizeX(vector<int>& deck) {
        unordered_map<int, int> m;
        for (int & i : deck)
            m[i]++;
        int g = 0;
        for (auto & p : m)
            g = gcd(p.second, g);
        return g > 1;
    }
};