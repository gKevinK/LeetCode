class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int r1 = m, r2 = n;
        while (r2 > 0) {
            if (r2 && (r1 == 0 || nums1[r1 - 1] < nums2[r2 - 1])) {
                nums1[r1 + r2 - 1] = nums2[(r2--) - 1];
            } else {
                nums1[r1 + r2 - 1] = nums1[(r1--) - 1];
            }
        }
    }
};