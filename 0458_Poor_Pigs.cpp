class Solution {
public:
    int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        int turns = minutesToTest / minutesToDie;
        return ceil(log(buckets) / log(turns + 1));
    }
};