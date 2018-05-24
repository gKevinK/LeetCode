class Solution {
public:
    bool isOneBitCharacter(vector<int>& bits) {
        int n = 0;
        for (int i = bits.size() - 2; i >= 0 && bits[i] == 1; i--)
            n++;
        return (n + 1) % 2;
    }
};