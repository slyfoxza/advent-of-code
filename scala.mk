CLEAN_SCALA := clean-scala

scala: scala.class
	$(file >$@,scala scala)
	@chmod 755 scala

scala.class: scala.scala
	scalac $<

clean-scala:
	$(RM) scala scala*.class
