class Solution {
public:
    int compress(vector<char>& chars) {
        int i = 0, j = 0;
        while (i < chars.size()) {
            chars[j] = chars[i];
            int k = 1;
            while (i + k < chars.size() && chars[i + k] == chars[i]) k++;
            if (k > 1)
                for (char & c : to_string(k))
                    chars[++j] = c;
            i += k;
            j++;
        }
        return j;
    }
};