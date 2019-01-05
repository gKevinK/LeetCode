class Solution {
public:
    int singleNonDuplicate(vector<int>& nums) {
        int last = INT_MIN, count = 0;
        for (int & n : nums) {
            if (last != n && count == 1)
                return last;
            if (last == n)
                count++;
            else {
                last = n;
                count = 1;
            }
        }
        return last;
    }
};