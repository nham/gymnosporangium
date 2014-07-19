# Unigraphs

For any set $X$, the collection $\mathcal{P}_k(X)$ is the collection of all subsets of $X$ consisting of exactly $k$ distinct elements. The sets in $\mathcal{P}_k(X)$ are called $k$-subsets.

An **undirected unigraph** is a tuple $(V, E)$ consisting of nodes $V$ and edges $E$, where $E$ is some collection of $1$- and $2$-subsets of $V$.

A **directed unigraph** is a tuple $(V, E)$ consisting of nodes $V$ and edges $E$, where $E$ is a subset of $V^2$.

A **unigraph** is either a directed or undirected unigraph.

Note that an undirected unigraph could be defined as a directed unigraph where for every edge $(i, j)$, $(j, i)$ is also an edge. In order to speak generically about edges in a unigraph, we will use this definition (so that we can just say, for example, that $(i, j)$ is an edge in a unigraph).


# Neighbors, degrees

For any unigraph $G$ and any node $i$ in $G$, $j$ is an **out-neighbor** iff there is an edge $(i, j)$ in $G$. Similarly, $k$ is an **in-neighbor** iff there is an edge $(k, i)$.

The **out-degree** of a node in a unigraph is the number of out-neighbors it has. Ditto for **in-degree** and in-neighbors.


# Walks, cycles

A **walk** in a unigraph is a finite sequence of nodes $(x_1, \ldots, x_n)$ such that for any $i \in \{1, \ldots, n-1\}$, $x_{i+1}$ is an out-neighbor of $x_i$.

A **sub-walk** of a walk $w$ is just a walk $x$ that is a sub-sequence of $w$.

A **cycle** is a walk of at least length 2 that starts and ends at the same node.

Consider the collection $W$ of all walks for a graph $G$. $G$ is said to be **cyclic** if $W$ contains at least one cycle, and **acyclic** otherwise.

It is easy to prove if $w$ is a walk for a unigraph $G$, then any sub-walk is also a walk for $G$.


## Proposition
For any unigraph $G$, if every node in $G$ has an out-neighbor, then $G$ is cyclic.

*Proof:* Since *every* node has an out-neighbor, we can construct walks of arbitrary length (for any walk, just append to it one of the out-neighbors of the last node). So there exists a walk whose length is greater than the number of nodes in the $G$. By the pigeonhole principle, one of the nodes must exist at two distinct steps in the walk, so we take the sub-walk starting and ending at those distinct steps to obtain a cycle.
