# Shunting yard playground

Provides a basic system for:

- loading shunting yards
- loading solutions
  - default solution is a graph based consisting of actions
- solution evaluation strategy
  - and basic evaluation

## Events:

- Train arrived
- Train departed
- Train moved
- Train washed
- Train inspected
- Train cleaned

- Train arrived wrongly
- Train arrived too late
- Train arrived too early
- Train not arrived
- Train unplanned arrival

Shunting layout is a graph
Shunting planning is a graph

Two different domains, and two different graphs.

Domain 1:
Shunting yard:

- rails(park, access)
- switches
- workstations(clean, inspect, wash)

At some facilities, the train needs to leave immediately after done.

## Note

Movements are important, but ... well... not that important? We should mainly focus on entries, services, splits, combines, and exits.
Like, we should probably just recalculate the movements during a check.

For example, we only add a move node to the graph. Then when calculating the conflicts, we build the solution to a proper form.

# Solution Graph

Since we don't know the movement times, we don't know any starting/ending times.

- A node starts after all parents are complete.
- Arrival nodes have a fixed start/end time.
- Departure nodes have a fixed start/end time.
- Movement nodes have start/end time only after evaluation and will start computation of routes. Multiple movements at the same time can't use the same tracks 
    - no tricks with using tracks that others are not using anymore, because the route has to be prepared entirely before walking.
- Service, Split, Combine nodes have start/end time only after evaluation
