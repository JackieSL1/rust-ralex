expression -> literal
            | unary
            | binary
literal    -> NUMBER | STRING
unary      ->
binary     -> expression operator expression;
operator   -> "=" | "<" | ">" | "select" condition | "project" condition
