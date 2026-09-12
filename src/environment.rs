struct Variable {
    t: todo!() // @work make variables
}
struct Alias {
    t: todo!() // @work, make this work
    // like remotely close to it
    // store as plaintext
    // eventually add flag aliases but that's in. a while
}
struct Env {
    variables: Vec<Variable>,
    aliases: Vec<Alias>
}