/**
 * Definition for undirected graph.
 * struct UndirectedGraphNode {
 *     int label;
 *     vector<UndirectedGraphNode *> neighbors;
 *     UndirectedGraphNode(int x) : label(x) {};
 * };
 */
class Solution {
public:
    UndirectedGraphNode *cloneGraph(UndirectedGraphNode *node) {
        unordered_map<UndirectedGraphNode*, UndirectedGraphNode*> map;
        queue<UndirectedGraphNode*> q;
        if (node == NULL) return NULL;
        map[node] = new UndirectedGraphNode(node->label);
        q.push(node);
        while (!q.empty()) {
            auto p = q.front();
            q.pop();
            for (auto p2 : p->neighbors) {
                if (map.find(p2) == map.end()) {
                    map[p2] = new UndirectedGraphNode(p2->label);
                    q.push(p2);
                }
                map[p]->neighbors.push_back(map[p2]);
            }
        }
        return map[node];
    }
};