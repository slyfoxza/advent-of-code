#!/usr/bin/env clojure
(defn the-business [offsets updater]
  (def u (dec (count offsets)))
  (loop [n 0 i 0 o offsets]
    (if (not (<= 0 i u))
      n
      (recur (inc n) (+ i (get o i)) (update-in o [i] updater)))))
(let [offsets (vec (map (fn [v] (Integer/parseInt v)) (line-seq (java.io.BufferedReader. *in*))))]
  (println
    (the-business offsets (fn [o] (inc o)))
    (the-business offsets (fn [o] (if (< o 3) (inc o) (dec o))))))
