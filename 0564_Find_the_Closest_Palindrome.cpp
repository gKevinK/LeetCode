class Solution {
public:
    string nearestPalindromic(string n) {
        long k = n.size(), N = stoll(n), r = 0;
        unordered_set<long> s = { static_cast<long>(pow(10, k)) + 1,
                                  static_cast<long>(pow(10, k - 1)) - 1 };
        int prefix = stoll(n.substr(0, (k + 1) / 2));
        for (long i : { prefix - 1, prefix, prefix + 1 }) {
            string sp = to_string(i);
            s.insert(stoll(sp + string(sp.rbegin() + (k & 1), sp.rend())));
        }
        s.erase(N);
        for (long i : s)
            if (abs(i - N) < abs(r - N) || (abs(i - N) == abs(r - N) && i < r))
                r = i;
        return to_string(r);
    }
};