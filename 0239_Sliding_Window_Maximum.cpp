class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        vector<int> res;
        list<int> lst;
        for (int i = 0; i < nums.size(); i++) {
            while (!lst.empty() && nums[lst.back()] < nums[i])
                lst.pop_back();
            lst.push_back(i);
            if (i >= k && lst.front() == i - k)
                lst.pop_front();
            if (i >= k - 1)
                res.push_back(nums[lst.front()]);
        }
        return res;
    }
};