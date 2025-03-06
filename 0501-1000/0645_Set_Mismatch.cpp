class Solution {
public:
    vector<int> findErrorNums(vector<int>& nums) {
        int r = 0, size = nums.size();
        for (int i = 1; i <= size; i++)
            r ^= i;
        for (int & n : nums)
            r ^= n;
        int m = 1;
        while ((r & m) == 0)
            m <<= 1;
        int r1 = 0, r2 = 0;
        for (int i = 1; i <= size; i++)
            (i & m ? r1 : r2) ^= i;
        for (int & n : nums)
            (n & m ? r1 : r2) ^= n;
        for (int & n : nums)
            if (r1 == n)
                return { r1, r2 };
        return { r2, r1 };
    }
};