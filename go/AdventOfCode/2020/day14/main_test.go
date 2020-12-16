package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleProgram = []Routine{
	{
		"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
		[]WriteOp{
			{8, 11},
			{7, 101},
			{8, 0},
		},
	},
}

func TestPart1(t *testing.T) {
	assert.Equal(t, uint64(165), RunProgram(exampleProgram))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 0, func() int { return 0 }())
}
