clean:
	rm -rfv step0_repl
	rm -rfv step1_read_print
step0_repl: clean
	rm step0_repl
	gcc -Wall -O2 -lreadline step0_repl.c -o step0_repl
step1_read_print: clean
	gcc -Wall -O2 -lreadline -lpcre step1_read_print.c -o step1_read_print
