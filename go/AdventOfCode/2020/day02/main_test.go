package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPart1(t *testing.T) {
	policies := []Policy{
		{1, 3, 'a', "abcde"},
		{1, 3, 'b', "cdefg"},
		{2, 9, 'c', "ccccccccc"},
	}
	assert.Equal(t, 2, CountValid(policies, 1))
}

func TestPart2(t *testing.T) {
	policies := []Policy{
		{1, 3, 'a', "abcde"},
		{1, 3, 'b', "cdefg"},
		{2, 9, 'c', "ccccccccc"},
	}
	assert.Equal(t, 1, CountValid(policies, 2))
}
