package main

import (
	"github.com/stretchr/testify/assert"
    "testing"
)

func TestPart1(t *testing.T) {
    image, err := parseImage("123456789012", 3, 2)
    assert.Nil(t, err)
    assert.Equal(t, 1, part1(image))
}

func TestImageString(t *testing.T) {
    image, err := parseImage("0222112222120000", 2, 2)
    assert.Nil(t, err)
    assert.Equal(t, " █\n█ ", image.String())
}
