#!/usr/bin/env ruby
require 'digest'

target = 5
key = ARGF.read.strip
1.step do |i|
  hash = Digest::MD5.hexdigest key + i.to_s
  if hash.start_with? '0' * target
    puts i
    break if target == 6
    target += 1
  end
end
# vim: set et sw=2 ts=2:
