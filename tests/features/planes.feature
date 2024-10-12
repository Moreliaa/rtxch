Feature: Planes

Scenario: The normal of a plane is constant everywhere
  Given p ← plane()
    And p1 ← point(0, 0, 0)
    And p2 ← point(10, 0, -10)
    And p3 ← point(-5, 0, 150)
  When n1 ← local_normal_at(p, p1)
    And n2 ← local_normal_at(p, p2)
    And n3 ← local_normal_at(p, p3)
  Then n1 = vector(0, 1, 0)
    And n2 = vector(0, 1, 0)
    And n3 = vector(0, 1, 0)

Scenario: Intersect with a ray parallel to the plane
  Given p ← plane()
    And pr ← point(0, 10, 0)
    And vr ← vector(0, 0, 1)
    And r ← ray(pr, vr)
  When xs ← local_intersect(p, r)
  Then xs.count = 0

Scenario: Intersect with a coplanar ray
  Given p ← plane()
    And pr ← point(0, 0, 0)
    And vr ← vector(0, 0, 1)
    And r ← ray(pr, vr)
  When xs ← local_intersect(p, r)
  Then xs.count = 0

Scenario: A ray intersecting a plane from above
  Given p ← plane()
    And pr ← point(0, 1, 0)
    And vr ← vector(0, -1, 0)
    And r ← ray(pr, vr)
  When xs ← local_intersect(p, r)
  Then xs.count = 1
    And xs[0].t = 1
    And xs[0].object = p

Scenario: A ray intersecting a plane from below
  Given p ← plane()
    And pr ← point(0, -1, 0)
    And vr ← vector(0, 1, 0)
    And r ← ray(pr, vr)
  When xs ← local_intersect(p, r)
  Then xs.count = 1
    And xs[0].t = 1
    And xs[0].object = p
