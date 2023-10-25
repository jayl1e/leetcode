package main

func isAlNum(c byte) bool {
	return c >= 'a' && c <= 'z' || c >= '0' && c <= '9' || c >= 'A' && c <= 'Z'
}

func iEq(l, r byte) bool {
	if l >= 'A' && l <= 'Z' {
		l += 'a' - 'A'
	}
	if r >= 'A' && r <= 'Z' {
		r += 'a' - 'A'
	}
	return l == r
}

func isPalindrome(s string) bool {
	ss := []byte(s)
	left, right := 0, len(ss)-1
	for left < right {
		if !isAlNum(ss[left]) {
			left += 1
		} else if !isAlNum(ss[right]) {
			right -= 1
		} else if !iEq(ss[left], ss[right]) {
			return false
		} else {
			left += 1
			right -= 1
		}
	}
	return true
}
