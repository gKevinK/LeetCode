class Solution {
public:
    string validIPAddress(string IP) {
        if (regex_match(IP, regex(R"==(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})=="))) {
            for (int i = 0; i < IP.size(); i++) {
                int j = i + 1;
                while (j < IP.size() && IP[j] != '.') j++;
                if (!regex_match(IP.substr(i, j - i), regex(R"==((0|[1-9]\d?|1\d{2}|2[0-4]\d|25[0-5]))==")))
                    return "Neither";
                i = j;
            }
            return "IPv4";
        } else if (regex_match(IP, regex(R"==(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4})=="))) {
            // for (int i = 0; i < IP.size();) {
            //     int j = i + 1;
            //     while (j < IP.size() && IP[j] != ':') j++;
            //     if (!regex_match(IP.substr(i, j - i), regex(R"==([0-9a-fA-F]{1,4})==")))
            //         return "Neither";
            //     for (i = j; i < IP.size() && IP[i] == ':'; i++);
            // }
            return "IPv6";
        }
        return "Neither";
    }
};