class Solution {
public:
    int lengthLongestPath(string input) {
        vector<int> v;
        int sum = 0, ts = 0;
        for (int i = 0; i < input.size(); ) {
            int t = 0, c = 0;
            bool file = false;
            while (input[i++] == '\t')
                t++;
            i--;
            while (i < input.size() && input[i++] != '\n') {
                c++;
                if (input[i - 1] == '.')
                    file = true;
            }
            i--;
            while (v.size() > t) {
                ts -= v.back() + 1;
                v.pop_back();
            }
            ts += c + 1;
            v.push_back(c);
            if (file)
                sum = max(sum, ts);
            i++;
        }
        return max(0, sum - 1);
    }
};