package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleInput = []Coord{
	{1, -2}, {1, -2}, {-1, 2}, {1, 2}, {1, 2}, {1, 2}, {-2, 0}, {1, -2}, {2, 0},
	{-1, -2}, {-2, 0}, {-1, -2}, {-1, -2}, {-2, 0}, {1, 2}, {1, 2}, {-2, 0},
	{1, -2}, {-2, 0}, {-1, -2}, {0, 0}, {1, 2}, {2, 0}, {2, 0}, {1, 2}, {1, -2},
	{-1, 2}, {-1, 2}, {-2, 0}, {-1, -2}, {1, 2}, {1, 2}, {-2, 0}, {-1, 2},
	{-2, 0}, {1, -2}, {-2, 0}, {1, 2}, {-1, 2}, {1, -2}, {-1, -2}, {2, 0},
	{-1, -2}, {0, 0}, {1, -2}, {-1, -2}, {1, 2}, {-1, -2}, {-1, -2}, {1, -2},
	{-1, 2}, {-2, 0}, {-1, 2}, {1, -2}, {0, 0}, {-1, 2}, {-1, 2}, {1, 2},
	{1, -2}, {2, 0}, {-1, -2}, {-1, -2}, {1, 2}, {1, 2}, {-2, 0}, {1, 2},
	{-1, -2}, {-2, 0}, {1, 2}, {-2, 0}, {1, -2}, {-1, -2}, {1, 2}, {1, -2},
	{2, 0}, {1, 2}, {0, 0}, {-1, -2}, {-2, 0}, {2, 0}, {-1, -2}, {1, 2},
	{-1, -2}, {1, 2}, {-1, 2}, {1, -2}, {-2, 0}, {-1, 2}, {1, 2}, {1, 2},
	{1, -2}, {2, 0}, {-1, 2}, {0, 0}, {2, 0}, {2, 0}, {1, -2}, {-1, 2}, {1, -2},
	{-1, -2}, {-1, -2}, {1, 2}, {-1, 2}, {-1, -2}, {-1, 2}, {-1, 2}, {1, -2},
	{-2, 0}, {-2, 0}, {-1, 2}, {1, -2}, {1, 2}, {0, 0}, {1, -2}, {-2, 0},
	{1, 2}, {1, 2}, {1, 2}, {1, 2}, {1, -2}, {-1, 2}, {1, -2}, {-2, 0}, {1, 2},
	{-1, 2}, {-2, 0}, {-2, 0}, {1, -2}, {0, 0}, {-2, 0}, {2, 0}, {-1, 2},
	{-2, 0}, {-2, 0}, {2, 0}, {1, -2}, {2, 0}, {2, 0}, {-2, 0}, {2, 0},
	{-1, -2}, {-2, 0}, {-2, 0}, {-1, 2}, {-2, 0}, {2, 0}, {0, 0}, {-2, 0},
	{-1, -2}, {2, 0}, {2, 0}, {1, -2}, {1, 2}, {1, 2}, {-2, 0}, {-1, 2},
	{-2, 0}, {-1, 2}, {1, -2}, {1, 2}, {-2, 0}, {1, -2}, {-1, 2}, {-2, 0},
	{1, -2}, {1, -2}, {1, -2}, {-1, 2}, {1, 2}, {0, 0}, {1, 2}, {2, 0},
	{-1, -2}, {1, -2}, {2, 0}, {-1, 2}, {-2, 0}, {-1, -2}, {-1, 2}, {-1, -2},
	{-1, -2}, {-1, 2}, {0, 0}, {1, 2}, {-1, 2}, {-1, -2}, {-2, 0}, {1, -2},
	{-2, 0}, {-1, -2}, {1, 2}, {1, 2}, {1, 2}, {-2, 0}, {1, -2}, {-1, 2},
	{1, -2}, {-1, 2}, {1, 2}, {1, -2}, {1, -2}, {1, 2}, {-2, 0}, {0, 0}, {2, 0},
	{1, 2}, {-2, 0}, {-1, 2}, {2, 0}, {-2, 0}, {1, 2}, {-1, -2}, {1, -2},
	{-2, 0}, {-1, 2}, {-1, -2}, {2, 0}, {-1, 2}, {2, 0}, {-1, -2}, {1, 2},
	{-1, 2}, {1, -2}, {-1, 2}, {-1, -2}, {0, 0}, {-1, -2}, {2, 0}, {1, 2},
	{-1, -2}, {1, 2}, {-1, -2}, {1, 2}, {1, 2}, {2, 0}, {-1, 2}, {1, 2},
	{-2, 0}, {2, 0}, {1, 2}, {-2, 0}, {-2, 0}, {1, 2}, {-1, -2}, {-1, -2},
	{1, 2}, {1, -2}, {0, 0}, {-1, -2}, {-2, 0}, {2, 0}, {1, -2}, {1, 2},
	{1, -2}, {-2, 0}, {2, 0}, {-1, 2}, {1, 2}, {-1, -2}, {-1, 2}, {-2, 0},
	{1, 2}, {1, -2}, {-1, -2}, {-2, 0}, {1, 2}, {0, 0}, {2, 0}, {1, 2}, {1, -2},
	{-1, 2}, {-1, -2}, {-2, 0}, {-1, -2}, {1, 2}, {1, 2}, {-1, -2}, {1, -2},
	{-1, 2}, {1, 2}, {-2, 0}, {-1, -2}, {1, -2}, {2, 0}, {-1, 2}, {1, -2},
	{1, -2}, {0, 0}, {-2, 0}, {-1, 2}, {1, 2}, {1, -2}, {1, 2}, {1, -2}, {1, 2},
	{-1, 2}, {-2, 0}, {1, 2}, {-1, 2}, {1, -2}, {-2, 0}, {2, 0}, {1, -2},
	{-2, 0}, {1, -2}, {1, -2}, {1, -2}, {-2, 0}, {0, 0}, {1, 2}, {1, 2},
	{-2, 0}, {-1, -2}, {-1, 2}, {2, 0}, {-2, 0}, {-1, -2}, {1, 2}, {1, 2},
	{1, -2}, {-1, 2}, {1, 2}, {1, -2}, {-2, 0}, {2, 0}, {-1, -2}, {0, 0},
	{2, 0}, {1, 2}, {-1, -2}, {-1, 2}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2},
	{-1, 2}, {-1, 2}, {-1, 2}, {-2, 0}, {1, -2}, {2, 0}, {-1, -2}, {1, 2},
	{2, 0}, {-2, 0}, {1, -2}, {1, 2}, {1, -2}, {0, 0}, {1, 2}, {-1, -2},
	{-1, 2}, {2, 0}, {-2, 0}, {-1, 2}, {-1, 2}, {1, -2}, {2, 0}, {-1, 2},
	{1, -2}, {2, 0}, {1, -2}, {-2, 0}, {1, -2}, {-1, 2}, {-1, -2}, {2, 0},
	{2, 0}, {-2, 0}, {2, 0}, {0, 0}, {-2, 0}, {1, -2}, {-2, 0}, {2, 0}, {2, 0},
	{2, 0}, {-1, 2}, {1, 2}, {1, -2}, {-1, 2}, {-2, 0}, {-2, 0}, {-1, -2},
	{1, 2}, {-2, 0}, {0, 0},
}

func TestPart1(t *testing.T) {
	assert.Equal(t, 10, CountPainted(exampleInput, false))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 2208, CountPainted(exampleInput, true))
}
