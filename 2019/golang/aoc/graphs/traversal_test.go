package graphs

import (
	"github.com/stretchr/testify/assert"
    "testing"
)

var adjacencyList = [][]interface {}{
    {1, 2},
    {3},
    {0, 3},
    {0, 1, 2},
}

func neighbors(x interface {}) []interface {} {
    return adjacencyList[int(x.(int))]
}

func TestSimpleBFTraversal(t *testing.T) {
    assert := assert.New(t)
    var values []int
    traversal := BFTraversal(0, neighbors)
    for node := traversal.Next(); node != nil; node = traversal.Next() {
        values = append(values, node.Value.(int))
    }
    assert.Equal(4, len(values))
    assert.Equal(0, values[0])
    assert.ElementsMatch([]int{1, 2}, values[1:3])
    assert.Equal(3, values[3])
}

func TestSimpleDFTraversal(t *testing.T) {
    assert := assert.New(t)
    var values []int
    traversal := DFTraversal(0, neighbors)
    for node := traversal.Next(); node != nil; node = traversal.Next() {
        values = append(values, node.Value.(int))
    }
    assert.Equal(4, len(values))
    assert.Equal(0, values[0])
    assert.Equal(3, values[2])
    assert.ElementsMatch([]int{1, 2}, []int{values[1], values[3]})
}
