package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleDepart int = 939

var exampleTimetable = []interface{}{
	7, 13, "x", "x", 59, "x", 31, 19,
}

func TestPart1(t *testing.T) {
	assert.Equal(t, 295, EarliestBus(exampleDepart, exampleTimetable))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 0, func() int { return 0 }())
}
