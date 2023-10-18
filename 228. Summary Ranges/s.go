package main

import "fmt"

func summaryRanges(nums []int) []string {
	var rt []string = nil
	if len(nums) == 0 {
		return nil
	}
	left, expected := nums[0], nums[0]
	var right int
	for _, num := range nums {
		if num == expected {
			expected += 1
		} else {
			right = expected - 1
			if right == left {
				rt = append(rt, fmt.Sprint(left))
			} else {
				rt = append(rt, fmt.Sprintf("%d->%d", left, right))
			}
			left = num
			expected = num + 1
		}
	}
	right = expected - 1
	if right == left {
		rt = append(rt, fmt.Sprint(left))
	} else {
		rt = append(rt, fmt.Sprintf("%d->%d", left, right))
	}
	return rt
}

func main() {
	fmt.Println(summaryRanges([]int{0, 1, 2, 4, 5, 7}))
}
