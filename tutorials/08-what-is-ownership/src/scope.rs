pub fn scope() {
    {                      // s is not valid here
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
        // this scope is over, and s is no longer valid
    }
}
