class Solution {
public:
    string getHint(string secret, string guess) {
        int A = 0, B = 0, len = secret.size();
        for (int i = 0; i < len; i++) {
            if (secret[i] == guess[i]) {
                A++;
                secret[i] = guess[i] = '*';
            }
        }
        map<char, int> map;
        for (char & c : secret) {
            if (c != '*')
                map[c]++;
        }
        for (char & c : guess) {
            if (map[c] > 0) {
                map[c]--;
                B++;
            }
        }
        return to_string(A) + "A" + to_string(B) + "B";
    }
};