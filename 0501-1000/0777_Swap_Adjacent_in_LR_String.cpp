class Solution {
public:
    bool canTransform(string start, string end) {
        int i = 0, j = 0;
        while (i < start.size() || j < start.size()) {
            while (i < start.size() && start[i] == 'X') i++;
            while (j < end.size() && end[j] == 'X') j++;
            if (start[i] != end[j] || (start[i] == 'R' && i > j) || (start[i] == 'L' && i < j))
                return false;
            i++;
            j++;
        }
        return true;
    }
};