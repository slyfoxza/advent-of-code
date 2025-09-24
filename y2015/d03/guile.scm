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

(define (hash-chained-set! table key val)
  (hash-set! table key val)
  table)

(define (make-visited-hash-table)
  (hash-chained-set! (make-hash-table) '(0 . 0) #t))

(define (point+ a b)
  (cons (+ (car a) (car b)) (+ (cdr a) (cdr b))))

(define (cond-point+ p a b)
  (if p
	(point+ a b)
	a))

(call-with-input-file
  "y2015/d03/input"
  (lambda (f)
	(let solve ((c (get-char f))
				(solo-santa-pos '(0 . 0))
				(santa-pos '(0 . 0))
				(robo-pos '(0 . 0))
				(is-robo #f)
				(visited1 (make-visited-hash-table))
				(visited2 (make-visited-hash-table)))
	  (if (eof-object? c)
		(values (hash-count (const #t) visited1)
				(hash-count (const #t) visited2))
		(let* ((dxy (cond ((char=? c #\^) '(0 . -1))
						  ((char=? c #\v) '(0 . 1))
						  ((char=? c #\<) '(-1 . 0))
						  ((char=? c #\>) '(1 . 0))
						  (else '(0 . 0))))
			   (next-solo-santa-pos (point+ solo-santa-pos dxy))
			   (next-santa-pos (cond-point+ (not is-robo) santa-pos dxy))
			   (next-robo-pos (cond-point+ is-robo robo-pos dxy)))
          (solve
			(get-char f)
			next-solo-santa-pos
			next-santa-pos
			next-robo-pos
			(not is-robo)
			(hash-chained-set! visited1 next-solo-santa-pos #t)
			(hash-chained-set! (hash-chained-set! visited2 next-santa-pos #t) next-robo-pos #t)))))))
