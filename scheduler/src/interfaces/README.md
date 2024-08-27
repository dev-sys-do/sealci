# Interface implementation

### src/interfaces/

Contains the gRPC interfaces implementation.
That is, handling requests, responses, streams...
There is no scheduler logic within that source code, only gRPC implementations, gRPC errors handling.

The only context known by this code is gRPC.
As such, this code should not handle any other errors than gRPC errors.

This code calls the Scheduler logic implementation defined in `src/logic/*`
All context relative to the Scheduler logic implementation (such as inputs from gRPC requests) is passed down to this code.
This is the heart of the Scheduler's logic.
This part of the code knows no context about the gRPC interfaces, or handling of errors regarding the gRPC interfaces.

## Agent logic

This program implements a priorty queue optimized for sorting operations (the most common operation for this data structure)

See the following:

- [Wikipedia: Priority queue](https://en.wikipedia.org/wiki/Priority_queue)
- [Wikipedia: Binary heap](https://en.wikipedia.org/wiki/Binary_heap)
- [Geeks for geeks: Binary heap](https://www.geeksforgeeks.org/binary-heap/)
- [Rust-lang docs: Binary heap](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html)

As for our more specific implementation, see the privately-shared (soon to come to the docs!) diagram on Excalidraw.
As per this diagram, the Agent pool handling logic is implemented through data structure II, and procedures 2, 3 and 4.
More detailed informations on the implementation are provided in:

- Part "II. Data structures",
- Part "III. Agent score calculation & Agent queue sorting",
- And part "IV. Log transfer".

Get to work!

## Controller logic

Not working on it yet.