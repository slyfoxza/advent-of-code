---
title: I Heard You Like Registers
date: 2017-12-08 14:55:48 -0500
---
# Perl
The default tab of examples shown on the [Perl 6 homepage][perl6home] shows off
its support for declaring grammars, so I decided to put that to the test with
this challenge.

[I think the result speaks for itself][perl6], especially when compared against
the monstrosities that were my [Rust Assembunny implementations from
2016][assembunny].

Now if the Perl 6 runtime packaged for Fedora 27 could run a bit faster, that'd
be great…

     Performance counter stats for './perl6.pl':

           6762.218365      task-clock:u (msec)       #    1.056 CPUs utilized
                     0      context-switches:u        #    0.000 K/sec
                     0      cpu-migrations:u          #    0.000 K/sec
               104,506      page-faults:u             #    0.015 M/sec
        19,280,451,696      cycles:u                  #    2.851 GHz
        18,896,176,077      instructions:u            #    0.98  insn per cycle
         4,042,840,383      branches:u                #  597.857 M/sec
           111,405,639      branch-misses:u           #    2.76% of all branches

           6.406334574 seconds time elapsed

[assembunny]: https://github.com/slyfoxza/advent-of-code/blob/96235cdbcbf1fa5d0a3cdf135d5d49df3a4100ae/2016/day25.rs
[perl6]: https://github.com/slyfoxza/advent-of-code/blob/96235cdbcbf1fa5d0a3cdf135d5d49df3a4100ae/2017/08/perl6.pl
[perl6home]: https://perl6.org/
