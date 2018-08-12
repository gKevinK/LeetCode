class Solution {
public:
    int candy(vector<int>& ratings) {
        int c[ratings.size()];
        int i, j;
        for (i = 0; i < ratings.size(); ++i) {
            c[i] = 1;
        }
        for (i = 0; i < ratings.size(); ++i) {
            if (i == 0 || i == ratings.size()-1 || (ratings[i-1] >= ratings[i] && ratings[i] <= ratings[i+1])) {
                j = i - 1;
                while (j >= 0 && ratings[j] > ratings[j+1]) {
                    c[j] = max(c[j], c[j+1] + 1); j--;
                }
                j = i + 1;
                while (j < ratings.size() && ratings[j] > ratings[j-1]) {
                    c[j] = max(c[j], c[j-1] + 1); j++;
                }
            }
        }
        int sum = 0;
        for (i = 0; i < ratings.size(); ++i) {
            sum += c[i];
        }
        return sum;
    }
};