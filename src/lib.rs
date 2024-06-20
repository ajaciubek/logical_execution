use graph::prelude::*;

pub struct LogicalExecutor {
    graph: DirectedCsrGraph<usize>,
    last_checkpoint: Option<usize>,
}

impl LogicalExecutor {
    pub fn new(edges: Vec<(usize, usize)>) -> Option<Self> {
        let graph: DirectedCsrGraph<usize> = GraphBuilder::new().edges(edges.clone()).build();

        // sanity check if graph has exactly one entry point and at least one exit point
        let result;
        if !LogicalExecutor::graph_sanity_check(&graph, &edges) {
            result = None;
        } else {
            result = Some(LogicalExecutor {
                graph,
                last_checkpoint: None,
            });
        }
        result
    }

    fn graph_sanity_check(graph: &DirectedCsrGraph<usize>, edges: &Vec<(usize, usize)>) -> bool {
        let mut no_of_start_nodes = 0;
        let mut no_of_end_nodes = 0;
        for (src, dest) in edges {
            if graph.in_degree(*src) == 0 {
                no_of_start_nodes += 1;
            }
            if graph.out_degree(*dest) == 0 {
                no_of_end_nodes += 1;
            }
        }

        if no_of_start_nodes != 1 {
            println!("Not exactly one root node");
            return false;
        }
        if no_of_end_nodes == 0 {
            println!("Missing end node");
            return false;
        }
        true
    }

    pub fn checkpoint(&mut self, checkpoint_id: usize) {
        match self.last_checkpoint {
            None => {
                if self.graph.in_degree(checkpoint_id) == 0 {
                    self.last_checkpoint = Some(checkpoint_id);
                } else {
                    panic!("wrong execution 1");
                }
            }
            Some(last_cp) => {
                if self
                    .graph
                    .out_neighbors(last_cp)
                    .any(|&node| node == checkpoint_id)
                {
                    if self.graph.out_degree(checkpoint_id) == 0 {
                        self.last_checkpoint = None;
                    } else {
                        self.last_checkpoint = Some(checkpoint_id);
                    }
                } else {
                    panic!("Wrong execution 2");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut le = LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)]).unwrap();
        le.checkpoint(0);
        le.checkpoint(1);
        le.checkpoint(2);
        le.checkpoint(4);
    }

    #[test]
    #[should_panic]
    fn test_2() {
        let mut le = LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)]).unwrap();
        le.checkpoint(1);
    }

    #[test]
    #[should_panic]
    fn test_3() {
        let mut le = LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)]).unwrap();
        le.checkpoint(0);
        le.checkpoint(1);
        le.checkpoint(4);
    }

    #[test]
    #[should_panic]
    fn test_4() {
        let mut le = LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)]).unwrap();
        le.checkpoint(0);
        le.checkpoint(1);
        le.checkpoint(2);
        le.checkpoint(4);
        le.checkpoint(4);
    }

    #[test]
    #[should_panic]
    fn test_5() {
        let mut _le =
            LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (2, 0)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_6() {
        let mut _le =
            LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (6, 1)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_7() {
        let mut _le =
            LogicalExecutor::new(vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 1)]).unwrap();
    }
}
