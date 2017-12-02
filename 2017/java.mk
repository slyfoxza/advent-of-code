.PHONY: clean-java

JAVA9_HOME ?= /usr/lib/jvm/java-9

java: java-target/aoc/aoc/AdventOfCode.class
	echo >java $(JAVA9_HOME)/bin/java --module-path java-target --module aoc/aoc.AdventOfCode
	[ -x java ] || chmod 755 java

java-target/aoc/aoc/AdventOfCode.class: java-source/aoc/AdventOfCode.java
	$(JAVA9_HOME)/bin/javac --module aoc --module-source-path java-source -d java-target

clean-java:
	$(RM) -r java java-target
