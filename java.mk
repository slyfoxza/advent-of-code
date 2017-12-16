CLEAN_JAVA := clean-java

java: java.class
	$(file >$@,java java)
	@chmod 755 $@

java.class: java.java
	javac $<

clean-java:
	$(RM) java java.class
