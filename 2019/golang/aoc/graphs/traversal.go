package graphs

type TraversalNode struct {
	Value  interface{}
	Depth  uint
	Index  uint
	Parent *TraversalNode
}

type graphTraversal struct {
	neighbors func(interface{}) []interface{}
	index     uint
	queue     nodeQueue
	seen      map[interface{}]bool
}

func BFTraversal(start interface{}, neighbors func(interface{}) []interface{}) graphTraversal {
	seen := make(map[interface{}]bool)
	seen[start] = true
    var queue nodeFIFOQueue
    queue.push(TraversalNode{Value: start, Depth: 0, Index: 0, Parent: nil})
	return graphTraversal{
		neighbors: neighbors,
		index:     0,
		seen:      seen,
		queue:     &queue,
	}
}

func DFTraversal(start interface{}, neighbors func(interface{}) []interface{}) graphTraversal {
	seen := make(map[interface{}]bool)
	seen[start] = true
    var queue nodeLIFOQueue
    queue.push(TraversalNode{Value: start, Depth: 0, Index: 0, Parent: nil})
	return graphTraversal{
		neighbors: neighbors,
		index:     0,
		seen:      seen,
		queue:     &queue,
	}
}

func (traversal *graphTraversal) Next() *TraversalNode {
    if traversal.queue.isEmpty() {
        return nil
    }
	node := traversal.queue.pop()
	for _, neighbor := range traversal.neighbors(node.Value) {
		traversal.index++
		if _, ok := traversal.seen[neighbor]; !ok {
			traversal.seen[neighbor] = true
			newNode := TraversalNode{
				Value:  neighbor,
				Index:  traversal.index,
				Depth:  node.Depth + 1,
				Parent: &node,
			}
			traversal.queue.push(newNode)
		}
	}
	return &node
}
