// Maybe a little bit misleading because we also have enums, but it'll work.

enum TokenTypes {
    MUL,
    DIV,
    ADD,
    SUB,
    STR,
    INT,
    FLT,
    EQL,
    KEY,
    NIL,
    CBT, // closed brackets (})
    OBT, // open brackets ({)
    OSB, // open square brackets ([)
    CSB, // closed square brackets (])
    OP, // open parenthesis (())
    CLP, // closed parenthesis (), added L for legal reasons
    OPS, // opposite (!)
    CMA, // comma (,)
}

enum VariableTypes {
    INT,
    FLT,
    STR,
    BLN, // boolean
    NIL, // nil value
    INF, // infinite value

}

struct Token {
    token_types: TokenTypes,
    body: String
}

// TODO: Implement variables during compile time
struct Variable {
    strictly_typed: bool, // If true, then the variable must be of a certain type.
    name: String,
    value: String
}