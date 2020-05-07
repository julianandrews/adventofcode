package graphs

type nodeQueue interface {
    pop() TraversalNode
    push(TraversalNode)
    isEmpty() bool
}

type nodeLIFOQueue []TraversalNode

func (queue *nodeLIFOQueue) push(node TraversalNode) {
    *queue = append(*queue, node)
}

func (queue *nodeLIFOQueue) pop() TraversalNode {
    *queue = (*queue)[:len(*queue) - 1]
    return (*queue)[0]
}

func (queue *nodeLIFOQueue) isEmpty() bool {
    return len(*queue) == 0
}

// TODO: Use a less stupid FIFO queue implementation
type nodeFIFOQueue []TraversalNode

func (queue *nodeFIFOQueue) push(node TraversalNode) {
    *queue = append(*queue, node)
}

func (queue *nodeFIFOQueue) pop() TraversalNode {
    node := (*queue)[0]
    *queue = (*queue)[1:]
    return node
}

func (queue *nodeFIFOQueue) isEmpty() bool {
    return len(*queue) == 0
}

