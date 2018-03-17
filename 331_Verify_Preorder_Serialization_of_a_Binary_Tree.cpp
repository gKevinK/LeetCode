class Solution {
public:
    bool isValidSerialization(string preorder) {
        int n = 1;
        for (int i = 0; i < preorder.size(); i++) {
            if (n == 0)
                return false;
            if (preorder[i] != '#') {
                n++;
            } else {
                n--;
            }
            while (i < preorder.size() && preorder[i] != ',')
                i++;
        }
        return n == 0;
    }
};