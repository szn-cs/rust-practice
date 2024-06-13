pub trait AdjacencyMatrix {
    /*
     * matrix: Vec<Vec<usize>>   => weights as values; initialize with: vec![vec![0; NUM_VERTECIES]; NUM_VERTECIES]
     */
}

pub trait AdjacencyList {
    /*
    * list: Vec<LinkedList<(usize, usize)>>     => tuples of node index and weight
    OR
    * list: Vec<Vec<(usize, usize)>>             => better cache performance
    */

    // OR

    /*
     *  type EdgeValue = i32; // value of node
     *  type NodeValue = i32; // value of node
     *
     *  vertex: HashMap<u64, NodeValue>
     *  adjacency: HashMap<u64, Vec<(u64, EdgeValue)>>
     */
}
