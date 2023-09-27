CC = clang
TARGET  = clisp
FLAGS   = -std=c99 -Wall -pedantic -ledit -lm
SHELL = /usr/bin/bash

$(TARGET).out: src/main.c
	$(CC) src/main.c $(FLAGS) $(HEADERS) -o target/$(TARGET).out

run: $(TARGET).out
	./target/$(TARGET).out $(USER)

valgrind: target/$(TARGET).out
	valgrind -s --leak-check=full target/$(TARGET).out $(USER) 2> target/mem_analysis.log
	@echo "---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----"
	@cat target/mem_analysis.log | cut --complement -d' ' -f1 | uniq

clear:
	rm -f target/*.out
	clear && ls
