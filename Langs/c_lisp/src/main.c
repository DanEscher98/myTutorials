#include <stdio.h>
#include <stdlib.h>
#include <editline/readline.h>
#include "polish.h"
#include "lib.h"
#include "mpc.h"


int main(int argc, char** argv) {
  file2var(POLISH_GRAMMAR, "polish_grammar.txt");
  mpca_lang(MPCA_LANG_DEFAULT, POLISH_GRAMMAR,
            Number, Operator, Expr, Lispy);

  /* Print Version and Exit Information */
  puts("Lispy Version 0.1");
  puts("Press Ctrl+c to Exit\n");
  /* In a never ending loop */
  while (1) {
    /* Output our prompt and get input */
    char* input = readline("lispy> ");
    /* Add input to history */
    add_history(input);
    /* Attempt to parse the user input */
    mpc_result_t ret_ast;
    if (mpc_parse("<stdin>", input, Lispy, &ret_ast)) {
      /* On success print and delete the AST */
      mpc_ast_print(ret_ast.output);
      mpc_ast_delete(ret_ast.output);
    } else {
      /* Otherwise print and delete the Error */
      mpc_err_print(ret_ast.error);
      mpc_err_delete(ret_ast.error);
    }
    free(input);
  }

  /* Undefine and delete our parsers */
  mpc_cleanup(4, Number, Operator, Expr, Lispy);
  return EXIT_SUCCESS;
}
