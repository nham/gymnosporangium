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

It is easy to prove if $w$ is a walk for a unigraph $G$, then any sub-walk is also a walk for $G$.

The **distance** from a node $i$ to a node $j$ is the length of the smallest walk starting at $i$ and ending at $j$.

A **cycle** is a walk with at least length 3 distinct nodes that starts and ends at the same node.

Consider the collection $W$ of all walks for a graph $G$. $G$ is said to be **cyclic** if $W$ contains at least one cycle, and **acyclic** otherwise.

A node with no out-neighbors will be called a **leaf node**.


## Proposition
For any unigraph $G$, if every node in $G$ has an out-neighbor, then $G$ is cyclic.

*Proof:* Since *every* node has an out-neighbor, we can construct walks of arbitrary length (for any walk, just append to it one of the out-neighbors of the last node). So there exists a walk whose length is greater than the number of nodes in the $G$. By the pigeonhole principle, one of the nodes must exist at two distinct steps in the walk, so we take the sub-walk starting and ending at those distinct steps to obtain a cycle.


## Proposition
For any acyclic unigraph $G$, if $i$ is a node with no out-neighbors and $H$ is the unigraph formed by removing $i$, then $H$ is also acyclic.

*Proof:* If $H$ were not acyclic then there would be a walk $w$ of $H$ that is a cycle. $w$ must also be a walk of $G$, so $w$ is a cycle of $G$.

### Corollary
Any acyclic unigraph can be reduced to an empty unigraph by removing leaf nodes.

*Proof:* The last two propositions say that 1) every acyclic graph has a leaf node and 2) removing a leaf node from an acyclic graph results in another acyclic graph.


# Subgraphs
A graph $G = (V, E)$ is the **subgraph** of a graph $H = (W, F)$ iff $V \subseteq W$ and $E \subseteq F$. If $E$ consists of every edge in $F$ between nodes in $V$, then $G$ is said to be an **induced subgraph** or to be the subgraph **induced by $V$**.

# Connected component
An induced subgraph $H$ of $G$ is a **connected component** of $G$ iff there is a walk starting at any node and ending at any other node in $H$.

Note: everyone else seems to call this a "strongly connected component", but that's a bad name because it makes you think there's some other notion of connectedness in use. "Strongly connected component" is actually how you generalize "connected component" to digraphs, as far as I understand. It's the only notion of connectedness you generally want. Hence I call it simply "connected component" here.


# Trees
A **tree** is either the empty graph or a tuple $(T, i)$ where $T$ is a graph such that there is a unique walk from $i$ to any other node in $T$. The node $i$ is called the **root** of the tree.

Trees must be acyclic, since if there were a cycle starting and ending at node $j$, then there would not be a unique path from the root to $j$ (we could always append the cycle to get another path from root to $j$).

In a rooted tree, each node has a well-defined **level**, which is the length of the path from root to the node.


# Breadth-first search

A **breadth-first search** on a graph $G$ starting at node $i$ in $G$ is a routine that returns a tree $T$ with $i$ as the root such that $T$ contains exactly the nodes in the connected component of $G$ that $i$ is in and such that each node's level in $T$ is it's distance from $i$ in $G$.
