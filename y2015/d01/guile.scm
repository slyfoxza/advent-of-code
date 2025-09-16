#| Copyright 2025 Philip Cronje
 |
 | This file is part of my Advent of Code solution repository. It is free software: you can
 | redistribute it and/or modify it under the terms of the GNU General Public License as published
 | by the Free Software Foundation, either version 3 of the License, or (at your option) any later
 | version.
 |
 | This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 | without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 | GNU General Public License for more details.
 |
 | You should have received a copy of the GNU General Public License along with this repository. If
 | not, see <https://www.gnu.org/licenses/>. |#
(use-modules (ice-9 textual-ports))

(call-with-input-file
  "y2015/d01/input"
  (lambda (f)
	(let solve ((c (get-char f))
				(floor 0)
				(position -1)
				(i 0))
	  (if (eof-object? c)
		(values floor position)
		(let ((next-floor (+ floor (cond ((char=? c #\() 1)
										 ((char=? c #\)) -1)
										 (else 0)))))
		  (solve
		    (get-char f)
			next-floor
			(if (and (= position -1) (< floor 0)) i position)
			(+ i 1)))))))
