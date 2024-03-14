
#[derive(Debug)]
struct PageRank{
    alpha: f64,
    epsilon: f64,
    max_iter: u32,
    num_nodes: usize,
    num_edges: usize,
    out_degree: Vec<usize>,
    in_edges: Vec<Vec<usize>>,
    out_edges: Vec<Vec<usize>>,
    rank: Vec<f64>,
    new_rank: Vec<f64>,
}

impl PageRank {
    fn new(alpha: f64, epsilon: f64, max_iter: u32, num_nodes: usize, num_edges: usize, out_degree: Vec<usize>, in_edges: Vec<Vec<usize>>, out_edges: Vec<Vec<usize>>) -> PageRank {
        PageRank {
            alpha,
            epsilon,
            max_iter,
            num_nodes,
            num_edges,
            out_degree,
            in_edges,
            out_edges,
            rank: vec![1.0; num_nodes],
            new_rank: vec![0.0; num_nodes],
        }
    }

    fn rank(&mut self) {
        for _ in 0..self.max_iter {
            for i in 0..self.num_nodes {
                self.new_rank[i] = (1.0 - self.alpha) / self.num_nodes as f64;
                for j in 0..self.in_edges[i].len() {
                    let in_node = self.in_edges[i][j];
                    self.new_rank[i] += self.alpha * self.rank[in_node] / self.out_degree[in_node] as f64;
                }

            }
            let mut diff = 0.0;
            for i in 0..self.num_nodes {
                diff += (self.new_rank[i] - self.rank[i]).abs();
                self.rank[i] = self.new_rank[i];
            }
            if diff < self.epsilon {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank() {
        let alpha = 0.85;
        let epsilon = 1e-6;
        let max_iter = 100;
        let num_nodes = 6;
        let num_edges = 12;
        let out_degree = vec![2, 3, 2, 2, 3, 2];
        let in_edges = vec![vec![2, 3], vec![2, 3, 0], vec![0, 1], vec![0, 1], vec![1, 4, 5], vec![4, 5]];
        let out_edges = vec![vec![2, 3], vec![2, 3, 0], vec![0, 1], vec![0, 1], vec![1, 4, 5], vec![4, 5]];
        let mut pr = PageRank::new(alpha, epsilon, max_iter, num_nodes, num_edges, out_degree, in_edges, out_edges);
        pr.rank();
        pr.rank.iter().for_each(|x| println!("{}", x));

        // Add more assertions
        assert!(pr.rank[0] > 0.0);
        assert!(pr.rank[1] > 0.0);
        assert!(pr.rank[2] > 0.0);
        assert!(pr.rank[3] > 0.0);
        assert!(pr.rank[4] > 0.0);
        assert!(pr.rank[5] > 0.0);

        // Check that the ranks sum to approximately 1
        let sum: f64 = pr.rank.iter().sum();
        println!("{}", sum);
        assert!((sum - 1.0).abs() < 1e-6);
    }
}

/**
 * The PageRank algorithm is a link analysis algorithm that assigns a numerical weighting to each element of a hyperlinked set of documents, such as the World Wide Web, with the purpose of "measuring" its relative importance within the set. The algorithm may be applied to any collection of entities with reciprocal quotations and references. The numerical weight that it assigns to any given element E is referred to as the PageRank of E and denoted by PR(E).
 * Usecase:
 * Relationship and closenes measure between people in a social network or realworld.
 */