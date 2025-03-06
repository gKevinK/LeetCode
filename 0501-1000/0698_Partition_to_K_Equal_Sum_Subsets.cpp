class Solution {
    vector<int> ns, buckets;
    int l;
    
    bool f(int t) {
        if (t == ns.size()) {
            for (int i : buckets)
                if (i != l)
                    return false;
            return true;
        }
        for (int i = 0; i < buckets.size(); ++i) {
            buckets[i] += ns[t];
            if (buckets[i] <= l) {
                if (f(t + 1))
                    return true;
            }
            buckets[i] -= ns[t];
            if (buckets[i] == 0)
                break;
        }
        return false;
    }
    
public:
    bool canPartitionKSubsets(vector<int>& nums, int k) {
        int sum = accumulate(nums.begin(), nums.end(), 0);
        if (sum % k) return false;
        l = sum / k;
        ns = nums;
        buckets = vector<int>(k, 0);
        sort(ns.rbegin(), ns.rend());
        return f(0);
    }
};