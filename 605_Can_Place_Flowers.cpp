class Solution {
public:
    bool canPlaceFlowers(vector<int>& flowerbed, int n) {
        int c = !flowerbed[0], l = flowerbed.size(), r = 0;
        for (int i : flowerbed) {
            if (i) {
                r += (c - 1) / 2;
                c = 0;
            } else
                c++;
        }
        r += c / 2;
        return r >= n;
    }
};