#!/usr/bin/env node
var accum = 0, i = 0, negind = null;
process.stdin.on('readable', () => {
	while((c = process.stdin.read(1)) !== null) {
		if(c == '(') accum += 1;
		else if (c == ')') accum -= 1;
		if(++i && (accum < 0) && (negind === null)) negind = i;
	}
});
process.stdin.on('end', () => { process.stdout.write(`${accum} ${negind}\n`); });
