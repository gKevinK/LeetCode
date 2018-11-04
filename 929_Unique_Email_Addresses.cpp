class Solution {
public:
    int numUniqueEmails(vector<string>& emails) {
        unordered_set<string> s;
        for (string & email : emails) {
            string real;
            for (int i = 0; i < email.size(); i++) {
                if (email[i] == '.')
                    continue;
                else if (email[i] == '+')
                    while (email[i + 1] != '@')
                        i++;
                else if (email[i] == '@') {
                    real += email.substr(i);
                    break;
                } else
                    real += email[i];
            }
            s.insert(real);
        }
        return s.size();
    }
};