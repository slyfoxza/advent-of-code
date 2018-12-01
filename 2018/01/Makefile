.PHONY: clean

ruby.rb: ruby.lit
	lit -t $<
	lit -w --out-dir ../../docs/2018/01 $<
	sed -i '1i #!/usr/bin/env ruby' $@
	chmod 755 $@

clean:; $(RM) ruby.rb
