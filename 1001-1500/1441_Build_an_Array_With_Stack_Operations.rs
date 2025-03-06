impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut op = Vec::new();
        let mut c = 1;
        for i in target {
            while c < i {
                op.push(String::from("Push"));
                op.push(String::from("Pop"));
                c += 1;
            }
            op.push(String::from("Push"));
            c += 1;
        }
        op
    }
}