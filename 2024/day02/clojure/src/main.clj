(ns main
  (:require [clojure.string :as str]))

(defn parse-report [line]
  (map Integer/parseInt
       (str/split line #" ")))

(defn check-report [report] ; NOTE: no check for empty
  (loop [dir nil prev (first report) r (rest report)]
    (if (empty? r) true
        (if dir
          ; dir is not nil
          (let [diff (case dir
                       :inc (- (first r) prev)
                       :dec (-  prev (first r)))]
            (if (and (<= 1 diff) (<= diff 3))
              (recur dir (first r) (rest r))
              false))

          ; dir is nil => first iteration
          (if (< prev (first r))
            (recur :inc prev r)
            (recur :dec prev r))))))

; tries to remove a level to make an unsafe report safe
; really stupidly by trying them all
(defn fix-report [report]
  (loop [left nil x (first report) right (rest report)]
    (if (empty? right) (check-report left)
        (or (check-report (concat left right))
            ; NOTE: concat on list, that's kinda ugly...
            ; but I couldn't think of a way to do this any smarter,
            ; and the inputs aren't very large so whatever
            (recur (concat left [x]) (first right) (rest right))))))

(defn part1 [input]
  (->> input
       str/split-lines
       (map parse-report)
       (map check-report)
       (filter identity)
       count))

(defn part2 [input]
  (->> input
       str/split-lines
       (map parse-report)
       (map #(or (check-report %) (fix-report %)))
       (filter identity)
       count))

(defn run [{:keys [file]}]
  (let [input (str/trim (slurp file))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))

(defn -main [& args]
  (run {:file (first args)}))
