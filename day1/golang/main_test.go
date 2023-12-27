package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_getNumForLine(t *testing.T) {
	assert.Equal(t, 98, getNumForLine("nineight"))
}
