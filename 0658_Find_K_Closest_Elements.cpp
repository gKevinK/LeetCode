class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        int lo = 0, hi = arr.size(), n = arr.size();
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            if (arr[mi] < x)
                lo = mi + 1;
            else
                hi = mi;
        }
        while (hi - lo < k) {
            if (lo == 0)
                hi++;
            else if (hi == n)
                lo--;
            else if (arr[hi] - x < x - arr[lo - 1])
                hi++;
            else
                lo--;
        }
        return vector<int>(arr.begin() + lo, arr.begin() + hi);
    }
};