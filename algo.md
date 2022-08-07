# Morphorm Layout Algorithm

Compute the sum of non-flex nodes in the main axis, and the max of non-flex nodes in the cross axis.

## Determine main and cross space for flex items

### Determine the main free space
If the main axis is a definite size, use that;

### Determine main size

1. If the node has a  main size, use that.
2. If the node has an auto size:
    - If the node has content_size, use it to compute the main size.
    -  