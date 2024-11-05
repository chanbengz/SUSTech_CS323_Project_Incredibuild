%{

%}

digit [0-9]
digits {digit}+
hexadigit [0-9a-fA-F]
hexadigits {hexadigit}+
int {digits}|0x{hexadigits}
float {digits}("."{digits})?
printable [ -~]
char "'"{printable}"'"|"'\\x"{hexadigit}{hexadigit}"'"

letter [a-zA-Z]
letter_ {letter}|_
identifier {letter_}({letter_}|{digits})*
whitespace [ \t\n]+

lifetime "<'"[a-z]">"
%%

{whitespace} { /* ignore whitespace */ }
{number} { return NUMBER; }
{identifier} { return IDENTIFIER; }
"=" { return EQUALS; }
"<" { return LT; }

%%