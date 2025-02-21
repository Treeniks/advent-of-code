(ns main-test
  (:require [main :refer :all]
            [clojure.test :refer :all]))

(def example1 "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")

(deftest test-part1-ex1
  (is (= (part1 example1) 2)))

(deftest test-part2-ex1
  (is (= (part2 example1) 4)))

(deftest test-fix-report
  (is (and
       (fix-report '(1 3 2 4 5))
       (fix-report '(8 6 4 4 1))
       (not (fix-report '(1 2 7 8 9)))
       (not (fix-report '(9 7 6 2 1))))))

(deftest test-check-report
  (is (and
       (check-report '(7 6 4 2 1))
       (check-report '(1 3 6 7 9))
       (not (check-report '(1 2 7 8 9)))
       (not (check-report '(9 7 6 2 1)))
       (not (check-report '(1 3 2 4 5)))
       (not (check-report '(8 6 4 4 1))))))
