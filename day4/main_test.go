package main

import "testing"

func TestAdjPass(t *testing.T) {
	n := 123445
	if !test_adj(n) {
		t.Errorf("Expected pass: %d", n)
	}
}

func TestAdjFail(t *testing.T) {
	n := 125456
	if test_adj(n) {
		t.Errorf("Expected fail: %d", n)
	}
}

func TestIncPass(t *testing.T) {
	n := 123368
	if !test_inc(n) {
		t.Errorf("Expected pass: %d", n)
	}
}

func TestIncFail(t *testing.T) {
	n := 153368
	if test_inc(n) {
		t.Errorf("Expected fail: %d", n)
	}
}