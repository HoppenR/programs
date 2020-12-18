package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleLine1 = []byte("2*3+(4*5)")
var exampleLine2 = []byte("5+(8*3+9+3*4*3)")
var exampleLine3 = []byte("5*9*(7*3*3+9*3+(8+6*4))")
var exampleLine4 = []byte("((2+4*9)*(6+9*8+6)+6)+2+4*2")

func TestPart1(t *testing.T) {
	assert.Equal(t, 26, EvalLine(exampleLine1))
	assert.Equal(t, 437, EvalLine(exampleLine2))
	assert.Equal(t, 12240, EvalLine(exampleLine3))
	assert.Equal(t, 13632, EvalLine(exampleLine4))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 0, func() int { return 0 }())
}
