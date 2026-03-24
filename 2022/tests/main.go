package tests

import "testing"

func AssertEq(t *testing.T, val int, expected int) {
	if val != expected {
		t.Errorf("Expected %d, but got %d\n", expected, val)
	}
}

func AssertEqStr(t *testing.T, val string, expected string) {
	if val != expected {
		t.Errorf("Expected %s, but got %s\n", expected, val)
	}
}

