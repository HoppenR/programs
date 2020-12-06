package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleForms = []string{
	"abc",
	"a\nb\nc",
	"ab\nac",
	"a\na\na\na",
	"b",
}

func TestPart1(t *testing.T) {
	assert.Equal(t, 11, SumCounts(exampleForms, 1))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 6, SumCounts(exampleForms, 2))
}
