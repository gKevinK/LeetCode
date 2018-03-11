class NumArray {
private:
    vector<int> arr;
public:
    NumArray(vector<int> &nums) {
        arr.push_back(0);
        if (nums.size() == 0) return;
        for (int i = 0; i < nums.size(); i++) {
            arr.push_back(nums[i] + arr[i]);
        }
    }

    int sumRange(int i, int j) {
        return arr[j+1] - arr[i];
    }
};

// Your NumArray object will be instantiated and called as such:
// NumArray numArray(nums);
// numArray.sumRange(0, 1);
// numArray.sumRange(1, 2);