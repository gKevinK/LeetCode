class Solution {
public:
    vector<int> twoSum(vector<int>& numbers, int target) {
        unordered_map<int, int> map;
        for (int i = 0; i < numbers.size(); i++) {
            if (map.find(numbers[i]) == map.end())
                map[target - numbers[i]] = i;
            else
                return { map[numbers[i]] + 1, i + 1 };
        }
        return { 0, 0 };
    }
};