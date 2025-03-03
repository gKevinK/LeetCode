class Solution {
public:
    bool rotateString(string A, string B) {
        if (A.size() != B.size())
            return false;
        A = A + A;
        return A.find(B) != std::string::npos;
    }
};