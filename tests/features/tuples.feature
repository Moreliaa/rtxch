Feature: Tuples feature

    Scenario: A tuple with w=1.0 is a point
        Given a <- Tuples(4.3, -4.2, 3.1, 1.0)
        Then a.x = 4.3
            And a.y = -4.2
            And a.z = 3.1
            And a.w = 1.0
            And a is a point
            And a is not a vector

    Scenario: A tuple with w=0 is a vector
        Given a <- Tuples(4.3, -4.2, 3.1, 0.0)
        Then a.x = 4.3
            And a.y = -4.2
            And a.z = 3.1
            And a.w = 0.0
            And a is not a point
            And a is a vector

    Scenario: point() creates tuples with w=1
        Given p <- point(4.0, -4.0, 3.0)
        Then p = tuple(4.0, -4.0, 3.0, 1.0)

    Scenario: vector() creates tuples with w=0
        Given v <- vector(4.0, -4.0, 3.0)
        Then v = tuple(4.0, -4.0, 3.0, 0.0)