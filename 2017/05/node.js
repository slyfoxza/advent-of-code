#!/usr/bin/env node
input = "";
process.stdin.on('data', function(data) { input += data; });
process.stdin.on('end', function() {
	offsets1 = input.trim().split('\n').map(x => parseInt(x));
	offsets2 = offsets1.slice();
	n = [0, 0];
	for(i = 0; i >= 0 && i < offsets1.length;) {
		++offsets1[i];
		i += offsets1[i] - 1;
		++n[0];
	}
	for(i = 0; i >= 0 && i < offsets2.length;) {
		offset = offsets2[i];
		offsets2[i] += (offset >= 3) ? -1 : 1;
		i += offset;
		++n[1];
	}
	console.log(n.join(' '));
});
