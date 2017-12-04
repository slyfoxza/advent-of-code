#!/usr/bin/env perl6
my $n1 = 0, my $n2 = 0;
for $*IN.lines {
	$n1++ unless m/<<(\w+) >>.*<<$0>>/;
	$n2++ unless .words.map({ .comb.sort.join }).join(' ') ~~ m/<<(\w+) >>.*<<$0>>/;
}
say "$n1 $n2";
