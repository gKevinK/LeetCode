class Solution {
public:
    vector<int> countBits(int num) {
        int arr[16] = {0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4};
        vector<int> v;
        for (int i = 0; i <= num; i++) {
            v.push_back(arr[i % 16]);
            int j = i >> 4;
            while (j > 0) {
                v[i] += arr[j % 16];
                j = j >> 4;
            }
        }
        return v;
    }
};