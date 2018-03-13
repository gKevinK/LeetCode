class NumArray {
private:
    vector<int> n;
    vector<int> t;

public:
    NumArray(vector<int> &nums) {
        n.resize(nums.size() + 1);
        t.resize(nums.size() + 1);
        for (int i = 0; i < nums.size(); ++i) {
            update(i, nums[i]);
        }
    }

    void update(int i, int val) {
        int dif = val - n[i + 1];
        for (int j = i + 1; j < n.size(); j += (j & -j)) {
            t[j] += dif;
        }
        n[i + 1] = val;
    }

    int sumRange(int i, int j) {
        return get(j + 1) - get(i);
    }

    int get(int i) {
        int res = 0;
        for (int j = i; j > 0; j -= (j & -j)) {
            res += t[j];
        }
        return res;
    }
};


// Your NumArray object will be instantiated and called as such:
// NumArray numArray(nums);
// numArray.sumRange(0, 1);
// numArray.update(1, 10);
// numArray.sumRange(1, 2);