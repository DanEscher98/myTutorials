#include "mpc.h"

/* Create Some Parsers */
const mpc_parser_t* Number = mpc_new("number");
const mpc_parser_t* Operator = mpc_new("operator");
const mpc_parser_t* Expr = mpc_new("expr");
const mpc_parser_t* Lispy = mpc_new("lispy");

/* Define them with the following Language */
static const char* POLISH_GRAMMAR = NULL;
// static const char* POLISH_GRAMMAR = "\
//   number    : /-?[0-9]+/ ;\
//   operator  : '+' | '-' | '*' | '/' ;\
//   expr      : <number> | '(' <operator> <expr>+ ')' ;\
//   lispy     : /^/ <operator> <expr>+ /$/ ;\
// ";
