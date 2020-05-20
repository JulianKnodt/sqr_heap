# Square Heap

#### Priority Queue with better scaling

A square heap, as opposed to a binary heap is one where the number of children at each level is
squared. That is, the root has two childrem. Each of those two children has four children,
meaning a total of 8 children at the next level. Then 64 children for the next layer, and so
forth.

## But why?  The number of nodes when a tree is full at a given height is ```
nodes(h) = (0..h).fold(0, |acc, n| acc + 2**(n * (n+1)/2))
```

We can bound the number of nodes by the top value:
```
nodes(h) >= 2**(h * (h+1)/2)
```

Thus, we can approximate the height given the number of nodes as
```
n >= 2**(h * (h+1)/2)
log(n) >= (h * (h+1)/2) where log is base 2
2 log(n) >= h * (h+1)
0 >= h^2 + h - 2 log(n)
let k = 2 log(n)
h >= (-1 +/- sqrt(1 + 4k))/2 using the quadratic formula
h >= (-1 + sqrt(1 +4k))/2 since it must be that h > 0

Throwing out constants
h >~ sqrt(1 + 8 log(n))/2
```

Thus, we find that the height will increase incredibly slowly with respect to the number of
nodes. Intuitively, the cost of operations in binary heaps are incurred due to traversal up and
down the heap. Our heap attempts to mitigate that by making the heap shorter but fatter. Of
course, there is an additional cost now because there is more than one children that needs to
be checked. Thus, we also must count the number of comparisons as a function of height.

```
#comparisons(h) = (0..h).fold(0, |acc, n| acc + 2**n)
                = h * (h+1)/2;
```

Thus, we find the number of comparisons as a function of the number of nodes to be approximately
equal to:
```
#comparisons(n) ~ (1 + 8 log(n))/8 + other lower order terms
#comparisons(n) ~ log(n)
```

Thus we find the comparisons here are also on the order of log(n).
So... there was no point in doing this.

Well, we do find a point in doing this. The cost incurred as the size of a priority queue
increases actually starts to depend more on memory, since there are often large jumps in indices
between different heights because the index change is exponential. Thus, it becomes harder for
the memory to prefetch the index of the next child. For most use cases, this is never reached,
as the cache line will be big enough. At a certain point though, it will start to hit a ceiling
where it will be necessary to go back to main memory to fetch the next child.

We dodge that problem by having fewer levels, and instead relying more on linear scans. Linear
scans also make good use of memory prefetching, so we expect having linear scans to be cheaper
than having to do large amounts of pointer chasing after some point. "After some point" means
that we expect it to incur some cost, but after making the heap large enough it will become
cheaper.

The trade-off comes in a couple forms: First, there is a little bit higher memory overhead. For
a binary heap, using a given index we can compute the child or the parent. I couldn't come up
with an easy way to do that for this scheme(although it might exist), so we must track the depth
of the last child along with how wide we expect the last level to be.

There is also an additional computational trade-off because we can't easily compute the next
child's index. Thus, there are a lot more numerical operations that occur at each level. Some of
these can be reduced using lookup tables, but there is always a little more overhead here.

Both of these stem from the fact that there isn't an immediate way to compute the index of the
child or the parent given an arbitrary index. With that, it would nullify these downsides.

It's also possible adding look-up tables or the like would be more efficient, but that's less
fun now isn't it?

### I don't believe you, show me some proof.

You right, it's all just fun until you can actually show good evidence that what you claim is
true. Unfortunately, there is no direct way to compare two data structures and see which is
better.

Consider this benchmark which shows that in certain cases a square heap might
be better suited than a binary heap in a practical use case. If you don't believe that it's
actually useful, fair enough, I can't convince everyone.

Fix the size of the heap to be some constant `l`. Then, pop from the top of the heap, and push
the element back on the heap. For both binary heaps and square heaps, this will force a sift up
and down the heap for each iteration. Thus, this should test the raw speed of each without any
other interference.




