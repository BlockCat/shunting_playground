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

## Building the solution

We use the solution graph, and a solution table (Look up connected to nodes).

First pass:

Arrival:
    put time in table

Movement:
    Starts at latest end time of parrents.
    1. Find allowed movement.

Second pass:

Consolidate movements
    



