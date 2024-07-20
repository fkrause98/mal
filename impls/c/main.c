#include <stdio.h>
#include <stdlib.h>
#include <readline/readline.h>

char* READ(char* s)
{
     return s;
}

char* EVAL(char* s)
{
     return s;
}

char* PRINT(char* s)
{
     return s;
}

char* rep(char* s)
{
     return READ(EVAL(PRINT(s)));
}

int main(void)
{
     char *line;
     do {
       line = readline("user> ");
       puts(line);
     } while (line != NULL);
     return 1;
}
