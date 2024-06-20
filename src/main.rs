use logical_execution::*;

fn main() {
    let mut le = LogicalExecutor::new(vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]).unwrap();
    le.checkpoint(0);
    // let graph: DirectedCsrGraph<usize> = GraphBuilder::new()
    //     .csr_layout(CsrLayout::Sorted)
    //     .edges(vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)])
    //     .build();

    // assert_eq!(graph.node_count(), 4);
    // assert_eq!(graph.edge_count(), 5);

    // assert_eq!(graph.out_degree(0), 2);
    // assert_eq!(graph.in_degree(1), 1);

    // assert_eq!(graph.out_neighbors(1).as_slice(), &[2, 3]);
    // assert_eq!(graph.in_neighbors(1).as_slice(), &[0]);
}
