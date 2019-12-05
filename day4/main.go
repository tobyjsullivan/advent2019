package main

import "fmt"

func main() {
  rStart := 109165
  rEnd := 576723
  result := test_range(rStart, rEnd)

  fmt.Println(fmt.Sprintf("Result: %d", result))
}

func test_range(s int, e int) int {
  count := 0
  for i := s; i <= e; i++ {
    if test(i) {
      count++;
    }
  }
  return count
}

func test(n int) bool {
  return test_adj(n) && test_inc(n)
}

func test_adj(n int) bool {
  last := n % 10
  n /= 10
  for n > 0 {
    cur := n % 10
    n /= 10
    if cur == last {
      // Peek next
      nxt := n % 10
      if nxt != cur {
        return true;
      }

      for nxt == last {
        n /= 10
        nxt = n % 10
      }
    }
    last = cur
  }
  return false
}

func test_inc(n int) bool {
  last := n % 10
  n /= 10
  for n > 0 {
    cur := n % 10
    n /= 10
    if cur > last {
      return false
    }
    last = cur
  }
  return true
}
