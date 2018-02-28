class Solution {
public:
    char nextGreatestLetter(vector<char>& letters, char target) {
        char r = target;
        for (char & c : letters) {
            if ((c < r && c > target) || (r <= target && c > target) || (r <= target && c < r))
                r = c;
        }
        return r;
    }
};