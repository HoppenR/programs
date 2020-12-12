package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleBoard = [][]byte{
	{'L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'},
	{'L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'},
	{'L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'},
	{'L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'},
	{'L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'},
	{'L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'},
	{'.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'},
	{'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'},
	{'L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'},
	{'L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'},
}

func TestPart1(t *testing.T) {
	assert.Equal(t, 37, FindStagnation(exampleBoard))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 0, func() int { return 0 }())
}
