fn main(){
    let mut x = vec!["hello", "wold"];
    {
        let y = &x[0];
    }
    x.push("foo");
}