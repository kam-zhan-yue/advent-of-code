package tests

import "testing"

func AssertEq(t *testing.T, val int, expected int) {
	if val != expected {
		t.Errorf("Expected %d, but got %d\n", expected, val)
	}
}

