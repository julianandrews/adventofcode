package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestIsSimpleCandidate(t *testing.T) {
	assert.True(t, isSimpleCandidate(111111))
	assert.False(t, isSimpleCandidate(23450))
	assert.False(t, isSimpleCandidate(123789))
}

func TestIsCandidate(t *testing.T) {
	assert.True(t, isCandidate(112233))
	assert.False(t, isCandidate(123444))
	assert.True(t, isCandidate(111122))
}
