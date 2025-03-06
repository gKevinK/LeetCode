class Solution {
public:
    vector<int> lexicalOrder(int n) {
        vector<int> v;
        int c = 0;
        while (v.size() < n) {
            if (c * 10 <= n) {
                c = max(0, 10 * c - 1);
            } else if (c % 10 == 9 || c >= n) {
                while (c % 10 == 9 || c >= n)
                    c /= 10;
            }
            c++;
            v.push_back(c);
        }
        return v;
    }
};