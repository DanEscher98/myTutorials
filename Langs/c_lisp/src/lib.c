#include "lib.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int file2var(char* var, const char* filename) {
  FILE *fp;
  char *buffer = NULL;
  size_t buffer_size = 0;
  size_t num_read;

  fp = fopen(filename, "r");
  if (fp == NULL) {
      fprintf(stderr, "Error: failed to open file '%s'\n", filename);
      return 1;
  }

  /* Determine the size of the file */
  fseek(fp, 0, SEEK_END);
  buffer_size = ftell(fp);
  rewind(fp);

  /* Allocate a buffer to hold the file contents */
  buffer = (char*)malloc(buffer_size + 1);
  if (buffer == NULL) {
      fprintf(stderr, "Error: failed to allocate memory\n");
      fclose(fp);
      return 1;
  }

  /* Read the file into the buffer */
  num_read = fread(buffer, sizeof(char), buffer_size, fp);
  if (num_read < buffer_size) {
      fprintf(stderr, "Error: failed to read file '%s'\n", filename);
      free(buffer);
      fclose(fp);
      return 1;
  }

  /* Add a null terminator to the buffer */
  buffer[num_read] = '\0';

  /* Close the file */
  fclose(fp);

  /* Set the static const variable */
  var = buffer;

  return 0;
}
