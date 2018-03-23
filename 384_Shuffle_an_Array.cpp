class Solution {
    vector<int> v;
public:
    Solution(vector<int> nums) {
        v = nums;
    }
    
    /** Resets the array to its original configuration and return it. */
    vector<int> reset() {
        return v;
    }
    
    /** Returns a random shuffling of the array. */
    vector<int> shuffle() {
        vector<int> n(v);
        for (int i = 0; i < v.size(); i++) {
            int j = rand() % (v.size() - i) + i;
            swap(n[i], n[j]);
        }
        return n;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(nums);
 * vector<int> param_1 = obj.reset();
 * vector<int> param_2 = obj.shuffle();
 */