
enum Value {

}
struct Db {
    dict: Map<String, Value>,
}
impl Db {
    pub fn lookupKeyReadOrReply(&self,  key: &String, value: &String) -> Option<RObj> {
        if(self.expireIfNeeded(key) == true) {

        }
    }
    pub fn expireIfNeeded(key: &String) -> bool {
        return false;
    }
}