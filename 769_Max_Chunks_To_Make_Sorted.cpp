class Solution {
public:
    int maxChunksToSorted(vector<int>& arr) {
        int s = 0, m = 0;
        for (int i = 0; i < arr.size(); i++) {
            m = max(m, arr[i]);
            if (m == i)
                s++;
        }
        return s;
    }
};