class Solution {
public:
    int uniqueMorseRepresentations(vector<string>& words) {
        vector<string> code {".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."};
        set<string> s;
        for (auto & word : words) {
            string str = "";
            for (char & c : word) {
                str += code[c - 'a'];
            }
            s.insert(str);
        }
        return s.size();
    }
};