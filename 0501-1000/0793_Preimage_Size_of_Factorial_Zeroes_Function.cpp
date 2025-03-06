class Solution {
public:
    int preimageSizeFZF(int K) {
        vector<int> v(14, 1);
        for (int i = 1; i < v.size(); i++) {
            v[i] = v[i - 1] * 5 + 1;
        }
        for (int i = 13; i > 0; i--) {
            if (K % v[i] == v[i] - 1)
                return 0;
            K %= v[i];
        }
        return 5;
    }
};