pub trait AdjacencyMatrix {
    /*
     * matrix: Vec<Vec<usize>>   => weights as values; initialize with: vec![vec![0; NUM_VERTECIES]; NUM_VERTECIES]
     */
}

pub trait AdjacencyList {
    /*
     * list: Vec<LinkedList<(usize, usize)>>     => tuples of node index and weight
     * list: Vec<Vec<(usize, usize)>>     => better cache performance
     */
}
